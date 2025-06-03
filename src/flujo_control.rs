pub fn flujo_control() {
    //Expresiones if
    let number = 6;
    if number < 5 {
        println!("la condicion es verdadera -> true");
    } else {
        println!("condicion es falsa -> false")
    }

    if number != 0 {
        println!("numero distinto de cero");
    }

    //Manejo de multiples condiciones else if
    if number % 4 == 0 {
        println!("es divisible por 4");
    } else if number % 3 == 0 {
        println!("divisible por 3");
    } else if number % 2 == 0 {
        println!("divisible por 2");
    } else {
        println!("no es divisible por 4,3 o 2");
    }

    //if en una declaracion let
    let condicion = true;
    let num = if condicion { 5 } else { 6 };

    println!("el valor del numero es: {num}");

    //Bucles
    //loop
    let mut numero = 0;
    loop {
        numero += 1;

        if numero == 5 {
            println!("continua {numero}...");
            continue;
        } else if numero == 10 {
            println!("Fin!");
            break;
        }

        println!("numero: {numero}");
    }

    //Devolviendo valores de los bucles
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("el resultado es {result}");

    //Etiquetas de bucles
    let mut contar = 0;
    'counting_up: loop {
        println!("contar = {contar}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if contar == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        contar += 1;
    }

    println!("fin contar = {contar}");

    //Bucle while
    let mut x = 3;
    while x != 0 {
        println!("{x}");
        x -= 1;
    }

    println!("LIFTOFF -> DESPEGUEEE!!!");

    //Bucle while para recorrer los elementos de una coleccion
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("el valor es : {}", array[index]);
        index += 1;
    }

    //Bucle a traves de una coleccion con for

    for element in array {
        println!("the value is {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }

    println!("LIFTOFF!!!")
}
