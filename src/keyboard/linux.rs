use std::fs::File;
use std::os::fd::AsFd;
use memmap2::MmapMut;
use std::sync::Arc;
use wayland_client::{Connection, Dispatch, Proxy, QueueHandle};
use wayland_client::globals::GlobalList;
use wayland_client::protocol::{wl_compositor, wl_registry, wl_shm};
use wayland_client::protocol::wl_buffer::WlBuffer;
use wayland_client::protocol::wl_compositor::WlCompositor;
use wayland_client::protocol::wl_registry::Request::Bind;
use wayland_client::protocol::wl_shm::{Format, WlShm};
use wayland_client::protocol::wl_shm_pool::WlShmPool;
use wayland_client::protocol::wl_surface::WlSurface;
use crate::keyboard::Shared;

pub(super) struct PlatformCoalescedKeyboard {

}

impl PlatformCoalescedKeyboard {
    pub fn new(shared: &Arc<Shared>) -> Self {
        //todo!()
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
        pixel.copy_from_slice(&[0xFF, 0x00, 0x00, 0xFF]); // Red pixels with full alpha
    }

    (file, mmap)
}

struct AppData {
    compositor: Option<wl_compositor::WlCompositor>,
    shm: Option<wl_shm::WlShm>,
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
        // When receiving events from the wl_registry, we are only interested in the
        // `global` event, which signals a new available global.
        // When receiving this event, we just print its characteristics in this example.
        if let wl_registry::Event::Global { name, interface, version } = event {
            match interface.as_str() {
                "wl_compositor" => {
                    println!("Found compositor (v{})", version);
                    let compositor = registry.bind::<wl_compositor::WlCompositor,_,_>(name, version, qh, ());

                    state.compositor = Some(compositor);
                }
                "wl_shm" => {
                    println!("Found shm (v{})", version);
                    let shm: WlShm = registry.bind(name, version, qh, ());
                    state.shm = Some(shm.try_into().expect("Can't convert shm to WlShm"));
                }
                _ => println!("Unrecognized global: {} (v{})", interface, version),
            }
        }
    }
}

impl Dispatch<WlCompositor, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlCompositor, event: <WlCompositor as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        todo!()
    }
}

impl Dispatch<WlShm, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlShm, event: <WlShm as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        println!("Got shm event {:?}",event);
    }
}

impl Dispatch<WlSurface, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlSurface, event: <WlSurface as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        todo!()
    }   
}
impl Dispatch<WlShmPool, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlShmPool, event: <WlShmPool as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        todo!()
    }
}

impl Dispatch<WlBuffer, ()> for AppData {
    fn event(state: &mut Self, proxy: &WlBuffer, event: <WlBuffer as Proxy>::Event, data: &(), conn: &Connection, qhandle: &QueueHandle<Self>) {
        todo!()
    }
}


pub fn debug_window_show() {
    let conn = Connection::connect_to_env().expect("Can't connect to wayland environment");
    let display = conn.display();
    let mut event_queue = conn.new_event_queue();
    let qh = event_queue.handle();
    let _registry = display.get_registry(&qh, ());
    let mut app_data = AppData {
        compositor: None,
        shm: None,
    };
    println!("Advertised globals:");
    // To actually receive the events, we invoke the `roundtrip` method. This method
    // is special and you will generally only invoke it during the setup of your program:
    // it will block until the server has received and processed all the messages you've
    // sent up to now.
    //
    // In our case, that means it'll block until the server has received our
    // wl_display.get_registry request, and as a reaction has sent us a batch of
    // wl_registry.global events.
    //
    // `roundtrip` will then empty the internal buffer of the queue it has been invoked
    // on, and thus invoke our `Dispatch` implementation that prints the list of advertised
    // globals.
    event_queue.roundtrip(&mut app_data).unwrap();
    let compositor = app_data
        .compositor.as_ref()
        .expect("Compositor not advertised");
    let shm = app_data.shm.as_ref().expect("SHM not advertised");
    let surface = compositor.create_surface(&qh, ());
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
    // Mark the entire surface as damaged
    surface.damage(0, 0, 200, 200);
    surface.commit();

    println!("Window should be displayed. Running event loop...");


    loop {
        event_queue.blocking_dispatch(&mut app_data).unwrap();
    }
}

pub fn debug_window_hide() {
    todo!()
}
