mod sound;

fn main() {
    // absolute
    crate::sound::instrument::clarinet();

    // relative
    sound::instrument::clarinet();
}
