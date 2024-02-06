use rouille::Response;

fn main() {
    rouille::start_server("0.0.0.0:3000", move |_| Response::text("Hello, world!"));
}
