use crossbeam_channel::{Receiver, Sender};
use nih_plug::debug::*;
use nih_plug::param;
use nih_plug::prelude::*;
use parking_lot::RwLock;
use rosc::{OscMessage, OscMidiMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::thread;

struct DawOut {
    params: Arc<DawOutParams>,
    sender: Option<Sender<OscMessage>>,
    param1_dirty: Arc<AtomicBool>,
    param2_dirty: Arc<AtomicBool>,
}

impl Default for DawOut {
    fn default() -> Self {
        let param1_dirty = Arc::new(AtomicBool::new(false));
        let param2_dirty = Arc::new(AtomicBool::new(false));

        Self {
            params: Arc::new(DawOutParams::new(
                param1_dirty.clone(),
                param2_dirty.clone(),
            )),
            param1_dirty,
            param2_dirty,
            sender: None,
        }
    }
}

struct OscChannel {
    sender: Sender<OscMessage>,
    receiver: Receiver<OscMessage>,
}

impl Default for OscChannel {
    fn default() -> Self {
        let (sender, receiver) = crossbeam_channel::bounded(65_536);
        Self { sender, receiver }
    }
}

#[derive(Params)]
struct DawOutParams {
    //Persisted Settings
    #[persist = "osc_server_address"]
    osc_server_address: RwLock<String>,
    #[persist = "osc_server_port"]
    osc_server_port: RwLock<u16>,
    #[persist = "osc_address_base"]
    osc_address_base: RwLock<String>,
    #[persist = "flag_send_midi"]
    flag_send_midi: RwLock<bool>,

    //Exposed Params
    #[id = "param1"]
    param1: FloatParam,
    #[id = "param2"]
    param2: FloatParam,
    // #[id = "param3"]
    // param3: FloatParam,
    // #[id = "param4"]
    // param4: FloatParam,
    // #[id = "param5"]
    // param5: FloatParam,
    // #[id = "param6"]
    // param6: FloatParam,
    // #[id = "param7"]
    // param7: FloatParam,
    // #[id = "param8"]
    // param8: FloatParam,
}

impl DawOutParams {
    #[allow(clippy::derivable_impls)]
    fn new(param1_dirty: Arc<AtomicBool>, param2_dirty: Arc<AtomicBool>) -> Self {
        Self {
            osc_server_address: RwLock::new("127.0.0.1".to_string()),
            osc_server_port: RwLock::new(9000),
            osc_address_base: RwLock::new("daw-out".to_string()),
            flag_send_midi: RwLock::new(true),
            param1: FloatParam::new("param1", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_step_size(0.01)
                .with_callback(Arc::new(move |x| {
                    nih_log!("param1: {}", x);
                    param1_dirty.store(true, Ordering::Release)
                })),
            param2: FloatParam::new("param2", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_step_size(0.01)
                .with_callback(Arc::new(move |x| {
                    nih_log!("param2: {}", x);
                    param2_dirty.store(true, Ordering::Release)
                })),
            // param3: FloatParam::new("param3", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }).with_step_size(0.01).with_callback(Arc::new(|x| nih_log!("param3: {}", x))),
            // param4: FloatParam::new("param4", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }).with_step_size(0.01).with_callback(Arc::new(|x| nih_log!("param4: {}", x))),
            // param5: FloatParam::new("param5", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }).with_step_size(0.01).with_callback(Arc::new(|x| nih_log!("param5: {}", x))),
            // param6: FloatParam::new("param6", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }).with_step_size(0.01).with_callback(Arc::new(|x| nih_log!("param6: {}", x))),
            // param7: FloatParam::new("param7", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }).with_step_size(0.01).with_callback(Arc::new(|x| nih_log!("param7: {}", x))),
            // param8: FloatParam::new("param8", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 }).with_step_size(0.01).with_callback(Arc::new(|x| nih_log!("param8: {}", x))),
        }
    }
}

enum OscMidiMessageType {
    NoteOff = 0x80,
    NoteOn = 0x90,
    // PolyPressure = 0xA0,
    // ControlChange = 0xB0,
    // ProgramChange = 0xC0,
    // ChannelPressure = 0xD0,
    // PitchBend = 0xE0,
    // SystemExclusive = 0xF0,
}

impl Plugin for DawOut {
    const NAME: &'static str = "DAW Out";
    const VENDOR: &'static str = "gamingrobot";
    const URL: &'static str = "https://github.com/gamingrobot/daw-out";
    const EMAIL: &'static str = "";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const DEFAULT_NUM_INPUTS: u32 = 0;
    const DEFAULT_NUM_OUTPUTS: u32 = 0;

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;
    const SAMPLE_ACCURATE_AUTOMATION: bool = false;

    fn params(&self) -> Arc<dyn Params> {
        nih_log!("Params Called");
        self.params.clone() as Arc<dyn Params>
    }

    fn initialize(
        &mut self,
        _bus_config: &BusConfig,
        _buffer_config: &BufferConfig,
        _context: &mut impl ProcessContext,
    ) -> bool {
        nih_log!("Initialize Called");

        //Setup OSC client
        //TODO: cleanup, better error handling
        let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not bind to address");
        let ip_port = format!(
            "{}:{}",
            *self.params.osc_server_address.read(),
            *self.params.osc_server_port.read()
        );
        nih_log!("Connecting: {}", ip_port);
        socket.connect(ip_port).expect("Connection failed");
        nih_log!("Connected!");

        let osc_channel = OscChannel::default();
        self.sender = Some(osc_channel.sender);
        //TODO: when should we join?
        let _client_thread = thread::spawn(move || write_thread(socket, osc_channel.receiver));

        true
    }

    // /<osc_address_base>/param/<param_name>
    // /<osc_address_base>/midi

    fn process(
        &mut self,
        _buffer: &mut Buffer,
        context: &mut impl ProcessContext,
    ) -> ProcessStatus {
        //TODO: should OSC MIDI port always be 0?
        //TODO: handle empty osc_address_base
        //TODO: better error handling
        //TODO: support other midi event types
        //TODO: more generic param handling (less copy paste)
        let osc_address_base = self.params.osc_address_base.read();
        if let Some(sender) = &self.sender {
            //Process Dirty Params
            if self
                .param1_dirty
                .compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                sender
                    .send(OscMessage {
                        addr: format!("/{}/param/{}", *osc_address_base, self.params.param1.name()),
                        args: vec![OscType::Float(self.params.param1.value)],
                    })
                    .unwrap();
            }

            if self
                .param2_dirty
                .compare_exchange(true, false, Ordering::Acquire, Ordering::Relaxed)
                .is_ok()
            {
                sender
                    .send(OscMessage {
                        addr: format!("/{}/param/{}", *osc_address_base, self.params.param2.name()),
                        args: vec![OscType::Float(self.params.param2.value)],
                    })
                    .unwrap();
            }

            //Process Note Events
            while let Some(event) = context.next_event() {
                nih_log!("Event: {:?}", event);
                match event {
                    NoteEvent::NoteOn {
                        timing: _,
                        channel,
                        note,
                        velocity,
                    } => sender
                        .send(OscMessage {
                            addr: format!("/{}/midi", *osc_address_base),
                            args: vec![OscType::Midi(OscMidiMessage {
                                port: 0,
                                status: (OscMidiMessageType::NoteOn as u8 | channel),
                                data1: note,
                                data2: (velocity * 127.0) as u8,
                            })],
                        })
                        .unwrap(),
                    NoteEvent::NoteOff {
                        timing: _,
                        channel,
                        note,
                        velocity,
                    } => sender
                        .send(OscMessage {
                            addr: format!("/{}/midi", *osc_address_base),
                            args: vec![OscType::Midi(OscMidiMessage {
                                port: 0,
                                status: (OscMidiMessageType::NoteOff as u8 | channel),
                                data1: note,
                                data2: (velocity * 127.0) as u8,
                            })],
                        })
                        .unwrap(),
                    _ => {}
                }
            }
        }

        ProcessStatus::Normal
    }
}

fn write_thread(socket: UdpSocket, recv: Receiver<OscMessage>) -> () {
    //TODO: remove expects
    while let Some(message) = recv.recv().ok() {
        let packet = OscPacket::Message(message);
        let buf = rosc::encoder::encode(&packet).expect("Bad OSC Data");
        let len = socket.send(&buf[..]).expect("Failed to send data");
        if len != buf.len() {
            nih_log!("UDP packet not fully sent");
        }
        nih_log!("Sent {:?} packet", packet);
    }
}

impl ClapPlugin for DawOut {
    const CLAP_ID: &'static str = "com.gamingrobot.daw-out";
    const CLAP_DESCRIPTION: &'static str = "Outputs MIDI/OSC information from the DAW";
    const CLAP_FEATURES: &'static [&'static str] = &["note_effect", "utility"];
    const CLAP_MANUAL_URL: &'static str = Self::URL;
    const CLAP_SUPPORT_URL: &'static str = Self::URL;
    const CLAP_HARD_REALTIME: bool = true;
}

impl Vst3Plugin for DawOut {
    const VST3_CLASS_ID: [u8; 16] = *b"grbt-daw-outputs";
    const VST3_CATEGORIES: &'static str = "Instrument|Tools";
}

nih_export_clap!(DawOut);
nih_export_vst3!(DawOut);
