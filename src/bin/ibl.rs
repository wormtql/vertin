use vertin::pbr::SchlickFresnelBaking;

fn main() {
    let baker = SchlickFresnelBaking::new(512, 1024);

    let result = baker.bake();

    result.save("image.png").unwrap();
}
