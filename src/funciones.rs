pub fn function() {
    fn another_function() {
        println!("otra funcion...");
    }

    another_function();

    //PARAMETROS, ARGUMENTOS
    fn valor(x: i32) {
        println!("Valor de x es {x}");
    }

    valor(5);

    fn mas_parametros(x: i32, letter: char) {
        println!("los valores son {x} y la letra {letter}");
    }

    mas_parametros(10, 'A');

    //SENTENCIAS Y EXPRESIONES
    //Sentencias son instrucciones que realizan alguna acción y no devuelven un valor.
    //Expresiones evalúan a un valor resultante.

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    //FUNCIONES CON VALORES DE RETORNO
    fn five() -> i32 {
        5
    }

    let x = five();
    println!("el valor es {x}");

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    let y = plus_one(5);

    println!("el valor es {y}");
}
