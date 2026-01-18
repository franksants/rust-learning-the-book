use std::io;
// aqui o std importa o io com o conceito do ::
// o :: dar uma ordem para o "use", ele basicamente pede para entrar
// em tal diretorio e importar um modulo ou diretorio com base no
// ultimo parametro

fn main() {
    let mut guess = String::new();
    // aqui definido uma variavel mutavel
    // do tipo string.
    println!("GUESS THE NUMBER");
    println!("Give the number here: ");

    //aqui usamos o stdin, que eh um modulo que estar dentro do diretorio "std::io"
    io::stdin()
        .read_line(&mut guess)
        .expect("There is a problem here.");

    //aqui provavelmente eh um format, onde esta a "{}" eh 
    //onde vai ser implementado o parametro.
    println!("The number guessed is {}", guess);
}


