fn main() {
    slint_build::compile("src/presentation/slint/app.slint")
        .expect("Erro ao compilar o arquivo Slint");
}
