use supertux_rs::run;

fn main() {
    pollster::block_on(run());
}
