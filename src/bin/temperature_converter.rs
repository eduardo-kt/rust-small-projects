use rust_small_projs::temperature::{processar_entrada, temperature::{Temperature, TemperatureScale}};


fn main() {
    println!("Conversor de Temperaturas");

    let mut temperatura: Temperature = loop {
        println!("Digite uma temperatura: ");
        let temperatura_usuario: f64 = match processar_entrada().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada de temperatura inválida");
                continue;
            }
        };

        println!("Escolha uma escala: ");
        println!("1- Celsius, 2-Fahrenheit, 3-Kelvin");
        let escala_usuario = match processar_entrada().as_str() {
            "1" => TemperatureScale::Celsius,
            "2" => TemperatureScale::Fahrenheit,
            "3" => TemperatureScale::Kelvin,
            _ => {
                println!("Escolha uma escala válida");
                continue;
            }
        };

        match Temperature::new(temperatura_usuario, escala_usuario) {
            Ok(temp) => break temp,
            Err(e) => {
                println!("Erro ao criar temperatura: {:?}", e);
                continue;
            }
        }
    };
    
    println!("{:?}", temperatura);
    
    loop {
        println!(r#"
        Escolha uma opção de conversão:
        1 para converter para Celsius
        2 para converter para Fahrenheit
        3 para converter para Kelvin
        q para sair
        "#);

        match processar_entrada().to_lowercase().as_str() {
            "1" => {
                temperatura = temperatura.convert_temperature_to(TemperatureScale::Celsius);
                println!("{:?}", temperatura);
            },
            "2" => {
                let temperatura = temperatura.convert_temperature_to(TemperatureScale::Fahrenheit);
                println!("{:?}", temperatura);
            },
            "3" => {
                let temperatura = temperatura.convert_temperature_to(TemperatureScale::Kelvin);
                println!("{:?}", temperatura);
            },
            "q" => {
                println!("Fechando aplicativo...");
                break;
            },
            _ => {
                println!("Entre com uma opção válida");
                continue;
            }
        };
    }
}