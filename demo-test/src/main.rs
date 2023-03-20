/// This example shows how to describe the adapter in use.
async fn run() {
    let instance = wgpu::Instance::default();

    let adapter = {
        instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap()
    };

    println!("{:?}", adapter.get_info());
}

fn main() {
    pollster::block_on(run());
}
