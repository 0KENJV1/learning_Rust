pub fn variables() {
    let patata = "patata";
    println!("i eat {patata}");

    let mut x = 5;
    println!("the value of x is -> {x}");
    x = 10;
    println!("the value of x is -> {x}");

    //constantes
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;
    println!("three hours in seconds: {}", THREE_HOURS_IN_SECONDS);

    //shadowing
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is -> {x}")
    }

    println!("the value of x is: {x}");

    //let mut spaces = " "; -> error
    let spaces = " ";
    let spaces = spaces.len();

    println!("{spaces}");

    //TIPOS DE DATOS
    //ENTEROS SIGNED
    let entero_signed = -8;
    let entero_unsigned: u32 = 8; // se supone que deberia mostrar u32

    println!("signo {}, sin signo {}", entero_signed, entero_unsigned);

    //FLOTANTES
    let flotante = 2.0;
    let flotante_menor: f32 = 3.0;

    println!(
        "flotante f64: {:#.1}, flotante f32: {:#.1}",
        flotante, flotante_menor
    );

    //OPERACIONES NUMERICAS
    //ADICION
    let sum = 5 + 10;
    println!("suma -> {sum}");

    //SUSTRACCION
    let difference = 95.5 - 4.3;
    println!("resta -> {difference}");

    //MULTIPLICATION
    let product = 4 * 32;
    println!("multiplicacion -> {product}");

    //DIVISION
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    println!("division -> {}, otra -> {}", quotient, truncated);

    //RESTO
    let remainder = 43 % 5;
    println!("resto -> {}", remainder);

    //BOOLEANO
    let t = true;
    let f = false;

    println!("false: {f}, true: {t}");

    //CHAR CARACTER
    let c = 'a';
    let z = 'Z';
    let heart_eyed_cat = '😻';

    println!("cararteres -> [{c}, {z}, {}]", heart_eyed_cat);

    //TIPOS COMPUESTOS
    //TUPLAS: Las tuplas tienen una longitud fija: una vez declaradas, su tamaño no puede aumentar ni disminuir.
    let mut tup = (500, 6.4, 1, true); //puede tener valores de varios tipos

    println!("tupla {:#?}", tup);
    //desestructuracion
    let tup_coordenadas = (2, 5.6, 1);
    let (_x, y, _z) = tup_coordenadas;
    println!("the value of y is: {y}");

    //acceder a los valores
    let indice = tup.0;
    println!("valor del indice 0 es -> {}", indice);

    tup.0 = 2;

    println!("tupla {:#?}", tup);

    //ARREGLOS: A diferencia de los arreglos en algunos otros lenguajes, los arreglos en Rust tienen una longitud fija.
    let mut arr = [1, 2, 3, 4, 5]; //solo pede tener valores de un unico tipo
    println!("array -> {:?}", arr);

    let arr2 = [3; 5];
    println!("array -> {:?}", arr2);

    //acceder a los valores
    let indice_arr = arr[0];
    println!("el valor del indice 0 es: {}", indice_arr);

    arr[0] = 2;
    println!("array -> {:?}", arr);
}
