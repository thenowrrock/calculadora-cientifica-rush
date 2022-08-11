use regex::Regex;
use std::io::{stdin};

/**
            * Es momento de desarrollar tu primera gran aplicación en Rust. Eres libre de idear y programar una aplicación de consola como una calculadora o un calendario. Para esto, ten presenta algunos consejos:

            Utilización de dependencias en Rust
            Explora la gran cantidad de dependencias que tiene Rust en su repositorio para encontrar las que necesites para tu app.

            Algunas dependencias interesantes:

            Expresiones regulares
            Manejo de fechas
            Generar números aleatorios
            Recuerda agregar la dependencia en el archivo Cargo.toml para su posterior descarga e importarla con use.

            use regex::Regex;

            fn main() {
                // ...
            }
            Cada dependencia tiene su propia documentación para que sepas cómo utilizarla.

            Conversiones de datos en Rust
            Veamos algunas formas de convertir datos en Rust que te servirá de ahora en adelante.

            String a entero
            Hay varias formas de declarar una variable del tipo String:

            let nombre = "123".to_string();

            let nombre: String = "123".to_string();

            let nombre = String::from("123");

            let mut nombre: String = String::new();
            nombre = "123".to_string();
            Conviértelo a número entero con parse(), pero teniendo en cuenta de borrar los espacios en blanco con trim() para evitar inconvenientes y capturar los errores con unwrap().

            let number: i32 = nombre.trim().parse().unwrap();
            println!("{}", number+1);
            String vs. &str
            Recuerda que el tipo de dato String permite manipular una cadena de texto, mientras que el tipo de dato &str contiene la referencia a un String, pero solo contiene su valor.
            Cuando le asignas a una variable un valor con dobles comillas "", las mismas crean un &str, por esto utilizamos to_string() para que retorne el tipo de dato en formato String y poder manipularlo. Y por este mismo motivo necesitas un String para hacer un .trim().parse() y convertir la variable al tipo entero.

            Para convertir un String a &str, solo agrega el & (Ampersand) delante del nombre de la variable o con as_str().

            let nombre: String = "123".to_string();
            let nombre_plano: &str = &nombre;

            let nombre: String = "123".to_string();
            let nombre_plano: &str = nombre.as_str();
            Muchas formas de hacer lo mismo ¿No te parece?

            TIP: Por convencion, en Rust, todas las variables y nombres de funciones que declares utilizan snackcase, o sea, un _ para separar las palabras. EJ: my_variable o my_function.

            
            Rust posee una curva de aprendizaje algo más compleja cuando se trata de tipos de datos y conversiones. Como siempre, la práctica hará que puedas comprender cuándo utilizar cada tipo y ser más veloz resolviendo problemas.
 * 
 * 
 * 
 */


fn main() {

    fn regex_addition(){
        //Regex regex_addition
        //(\d+)\s?\+\s?(\d+) expresion regular
        let re_add = Regex::new(r"(\d+)\s?\+\s?(\d+)").unwrap();

        //Datos usuario
        println!("Funcion Sumar,Por favor introduce tu expresion:");
        let mut expresion = String::new();
        stdin().read_line(&mut expresion).unwrap();

        //Validaciones
        //Capture funciona Some(Captures({0: Some("10 +22"), 1: Some("10"), 2: Some("22")}))
        loop{
            //Aplicar operaciones
            let caps = re_add.captures(expresion.as_str());
            if caps.is_none(){
                break;
            }
            let caps = caps.unwrap();
            let caps = re_add.captures(expresion.as_str()).unwrap();
            let cap_expresion = caps.get(0).unwrap().as_str();
            let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let righ_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let addition  = left_value + righ_value;

            expresion = expresion.replace(cap_expresion, &addition.to_string());       
        }
        //println!("{:?} left_value:{} righ_value:{}",caps,left_value,righ_value); //Para tener mas detalle que hace esta variable :?
        //Resultado
        println!("Resultado :{}",expresion);     
            
    }


    fn regex_subtraction(){
        //Regex regex_addition
        //(\d+)\s?\+\s?(\d+) expresion regular
        let re_sub = Regex::new(r"(\d+)\s?\-\s?(\d+)").unwrap();

        //Datos usuario
        println!("Funcion Restar,Por favor introduce tu expresion:");
        let mut expresion = String::new();
        stdin().read_line(&mut expresion).unwrap();

        //Validaciones
        //Capture funciona Some(Captures({0: Some("10 +22"), 1: Some("10"), 2: Some("22")}))
        loop{
            //Aplicar operaciones
            let caps = re_sub.captures(expresion.as_str());
            if caps.is_none(){
                break;
            }
            let caps = caps.unwrap();
            let caps = re_sub.captures(expresion.as_str()).unwrap();
            let cap_expresion = caps.get(0).unwrap().as_str();
            let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let righ_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let addition  = left_value - righ_value;

            expresion = expresion.replace(cap_expresion, &addition.to_string());       
        }
        //println!("{:?} left_value:{} righ_value:{}",caps,left_value,righ_value); //Para tener mas detalle que hace esta variable :?
        //Resultado
        println!("Resultado :{}",expresion);  
    }

    fn regex_multiply(){
        //Regex regex_addition
        //(\d+)\s?\+\s?(\d+) expresion regular
        let re_mul = Regex::new(r"(\d+)\s?\*\s?(\d+)").unwrap();

        //Datos usuario
        println!("Funcion Multiplicar,Por favor introduce tu expresion:");
        let mut expresion = String::new();
        stdin().read_line(&mut expresion).unwrap();

        //Validaciones
        //Capture funciona Some(Captures({0: Some("10 +22"), 1: Some("10"), 2: Some("22")}))
        loop{
            //Aplicar operaciones
            let caps = re_mul.captures(expresion.as_str());
            if caps.is_none(){
                break;
            }
            let caps = caps.unwrap();
            let caps = re_mul.captures(expresion.as_str()).unwrap();
            let cap_expresion = caps.get(0).unwrap().as_str();
            let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let righ_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let addition  = left_value * righ_value;

            expresion = expresion.replace(cap_expresion, &addition.to_string());       
        }
        //println!("{:?} left_value:{} righ_value:{}",caps,left_value,righ_value); //Para tener mas detalle que hace esta variable :?
        //Resultado
        println!("Resultado :{}",expresion);  
    }

    fn regex_split(){
        //Regex regex_addition
        //(\d+)\s?\+\s?(\d+) expresion regular
        let re_spl=Regex::new(r"(\d+)\s?/\s?(\d+)").unwrap();
        //Datos usuario
        println!("Funcion Dividir,Por favor introduce tu expresion:");
        let mut expresion = String::new();
        stdin().read_line(&mut expresion).unwrap();

        //Validaciones
        //Capture funciona Some(Captures({0: Some("10 +22"), 1: Some("10"), 2: Some("22")}))
        loop{
            //Aplicar operaciones
            let caps = re_spl.captures(expresion.as_str());
            if caps.is_none(){
                break;
            }
            let caps = caps.unwrap();
            let caps = re_spl.captures(expresion.as_str()).unwrap();
            let cap_expresion = caps.get(0).unwrap().as_str();
            let left_value: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
            let righ_value: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
            let addition  = left_value / righ_value;

            expresion = expresion.replace(cap_expresion, &addition.to_string());       
        }
        //println!("{:?} left_value:{} righ_value:{}",caps,left_value,righ_value); //Para tener mas detalle que hace esta variable :?
        //Resultado
        println!("Resultado :{}",expresion);  
    }

    fn imprimir(msg:&str){
        println!("{}",msg);
    }

    fn control_calculadora(){
        //Variables para control de menu
        let sum_str:&str = "sumar";
        let sub_str:&str = "restar";
        let mul_str:&str = "multiplicar";
        let div_str:&str = "dividir";
        let exit_str:&str = "exit";
        let sum:i8 = 1;
        let sub:i8 = 2;
        let mul:i8 = 3;
        let div:i8 = 4;
        let exit:i8 = 5;
        let line:&str = "------------------------------------------------------------";
        
        loop{
            //menu bar
            println!("Bienvenidos a la Calculadora Cientifica con Rush");
            println!("Funcionalidades : 1-{}, 2-{}, 3-{}, 4-{}, 5-{}",sum_str,sub_str,mul_str,div_str,exit_str);
            println!("Ingrese la opcion de la calculadora:");
            let mut opcion = String::new();
            stdin().read_line(&mut opcion).unwrap();
            let op_int:i8 = opcion.trim().parse().unwrap();

            if op_int == sum{
                regex_addition();
                imprimir(line);
            }
            else if op_int == sub{
                regex_subtraction();
                imprimir(line);
            }
            else if op_int == mul{
                regex_multiply();
                imprimir(line);
            }
            else if op_int == div{
                regex_split();
                imprimir(line);
            }
            else if op_int == exit{
                imprimir("Saliste de la aplicación, thanks you.");
                break;
            }
            else{
                imprimir("Termino o diferente es tu entrada. Final de ejecutar");
                break;
            }
        }
        //LLamando funciones
        // regex_addition();
        // regex_subtraction();
        // regex_multiply();
        // regex_split();
    }

    //Inicar calduladora
    control_calculadora();

}
