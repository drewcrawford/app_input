use std::fs::File;
use std::os::fd::AsFd;
use memmap2::MmapMut;
use std::sync::{Arc, Mutex, OnceLock, Weak};
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_client::globals::{registry_queue_init, GlobalList, GlobalListContents};
use wayland_client::protocol::{wl_compositor, wl_pointer, wl_registry, wl_shm};
use wayland_client::protocol::__interfaces::wl_keyboard_interface;
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_keyboard::WlKeyboard;
use wayland_client::protocol::wl_pointer::WlPointer;
use wayland_client::protocol::wl_seat::WlSeat;
use wayland_client::protocol::wl_shm::{Format, WlShm};
use wayland_client::protocol::wl_shm_pool::WlShmPool;
use wayland_client::protocol::wl_surface::WlSurface;
use wayland_protocols::xdg::shell::client::xdg_surface::XdgSurface;
use wayland_protocols::xdg::shell::client::xdg_toplevel::XdgToplevel;
use wayland_protocols::xdg::shell::client::xdg_wm_base::{Event, XdgWmBase};
use crate::keyboard::Shared;

struct KeyboardState {
    shareds: Vec<Weak<Shared>>
}
impl Default for KeyboardState {
    fn default() -> Self {
        KeyboardState {
            shareds: Vec::new()
        }
    }
}
impl KeyboardState {
    fn apply_all<F: Fn(&Shared) -> ()>(&mut self, f: F) {
        self.shareds.retain(|shared| {
            if let Some(shared) = shared.upgrade() {
                f(&shared);
                true
            } else {
                false
            }
        })
    }
}
static KEYBOARD_STATE: OnceLock<Mutex<KeyboardState>> = OnceLock::new();

pub(super) struct PlatformCoalescedKeyboard {

}

impl PlatformCoalescedKeyboard {
    pub fn new(shared: &Arc<Shared>) -> Self {
        KEYBOARD_STATE.get_or_init(Mutex::default).lock().unwrap().shareds.push(Arc::downgrade(shared));
        PlatformCoalescedKeyboard {

        }
    }
}

fn create_shm_buffer(
    shm: &wl_shm::WlShm,
    width: u32,
    height: u32,
) -> (File, MmapMut) {
    let stride = width * 4;
    let size = stride * height;
    let mut file = tempfile::tempfile().unwrap();
    file.set_len(size as u64).unwrap();

    let mut mmap = unsafe{MmapMut::map_mut(&file)}.unwrap();

    for pixel in mmap.chunks_exact_mut(4) {
        pixel.copy_from_slice(&[0, 0, 0xFF, 0xFF]); //I guess due to endiannness we are actually BGRA?
    }

    (file, mmap)
}

struct AppData {
}
impl Dispatch<wl_registry::WlRegistry, ()> for AppData {
    fn event(
        state: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<AppData>,
    ) {
        println!("Got registry event {:?}",event);
    }
}

impl Dispatch<WlCompositor, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlCompositor, event: <WlCompositor as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("Got compositor event {:?}",event);
    }
}

impl Dispatch<WlShm, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlShm, event: <WlShm as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("Got shm event {:?}",event);
    }
}

impl Dispatch<WlSurface, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlSurface, event: <WlSurface as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("got WlSurface event {:?}",event);
    }   
}
impl Dispatch<WlShmPool, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlShmPool, event: <WlShmPool as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("got WlShmPool event {:?}",event);
    }
}

impl Dispatch<WlBuffer, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlBuffer, event: <WlBuffer as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("got WlBuffer event {:?}",event);
    }
}

impl Dispatch<XdgWmBase, ()> for AppData {
    fn event(state: &mut Self, proxy: &XdgWmBase, event: <XdgWmBase as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            Event::Ping { serial } => {
                proxy.pong(serial)
            }
            _ => {
                println!("Unknown XdgWmBase event: {:?}", event); // Add this line

            }
        }
    }
}

impl Dispatch<XdgSurface, ()> for AppData {
    fn event(state: &mut Self, proxy: &XdgSurface, event: <XdgSurface as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("got XdgSurface event {:?}",event);
    }
}

impl Dispatch<WlSeat, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlSeat, event: <WlSeat as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("got WlSeat event {:?}",event);
    }
}

// You need to provide a Dispatch<WlRegistry, GlobalListContents> impl for your app
impl wayland_client::Dispatch<wl_registry::WlRegistry, GlobalListContents> for AppData {
    fn event(
        state: &mut AppData,
        proxy: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        // This mutex contains an up-to-date list of the currently known globals
        // including the one that was just added or destroyed
        data: &GlobalListContents,
        conn: &Connection,
        qhandle: &QueueHandle<AppData>,
    ) {
        println!("got registry event {:?}",event);
    }
}

impl Dispatch<XdgToplevel, ()> for AppData {
    fn event(state: &mut Self, proxy: &XdgToplevel, event: <XdgToplevel as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("got XdgToplevel event {:?}",event);
    }
}



impl Dispatch<WlPointer, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlPointer, event: <WlPointer as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            // wayland_client::protocol::wl_pointer::Event::Motion {time,surface_x,surface_y} => {
            //     motion_event(time, surface_x, surface_y);
            //
            // }
            _ => println!("got WlPointer event {:?}",event)
        }
    }
}

impl Dispatch<WlKeyboard, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlKeyboard, event: <WlKeyboard as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        match event {
            wayland_client::protocol::wl_keyboard::Event::Key {serial, time, key, state} => {
                println!("got key event {:?} {:?} {:?}",serial,time,key);
            }
            _ => println!("got WlKeyboard event {:?}",event)
        }
    }
}



pub fn debug_window_show() {
    let conn = Connection::connect_to_env().expect("Can't connect to wayland environment");
    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();
    let _registry = display.get_registry(&qh, ());
    let mut app_data = AppData {
    };
    let (globals, queue) = registry_queue_init::<AppData>(&conn).expect("Can't initialize registry");
    let xdg_wm_base: XdgWmBase = globals.bind(&qh, 6..=6, ()).unwrap();

    let compositor: wl_compositor::WlCompositor = globals.bind(&qh, 6..=6, ()).unwrap();
    let shm = globals.bind(&qh, 2..=2, ()).unwrap();


    let surface = compositor.create_surface(&qh, ());
    // Create a toplevel surface
    let xdg_surface = xdg_wm_base.get_xdg_surface(&surface, &qh, ());
    xdg_surface.get_toplevel(&qh, ());

    let (file, mmap) = create_shm_buffer(&shm, 200, 200);
    let pool = shm.create_pool(file.as_fd(), mmap.len() as i32, &qh, ());
    let buffer = pool.create_buffer(
        0,
        200,
        200,
        200 * 4,
        Format::Argb8888,
        &qh,
        (),
    );
    surface.attach(Some(&buffer), 0, 0);
    surface.commit();

    let seat: WlSeat = globals.bind(&qh, 6..=6, ()).expect("Can't bind seat");
    let pointer = seat.get_pointer(&qh, ());
    let keyboard = seat.get_keyboard(&qh, ());


    println!("Window should be displayed. Running event loop...");


    loop {
        event_queue.blocking_dispatch(&mut app_data).unwrap();
    }
}

pub fn debug_window_hide() {
    todo!()
}
