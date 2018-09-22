extern crate rand;

use std::io; //Подключение библиотеки i/o
use std::cmp::Ordering;
use rand::Rng; //Random number generator

fn main() {
    println!("Угадайте число!"); // Макрос, который выводит

    // Генерация случайного числа
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Введите предположение.");// строки на экран

        /*
        * let используется для создания связи (переменной)
        * по умолчанию связи неизменяемы, по этой причине
        * используется mut
        * mut модификатор позволяющий изменять связь
        * ::new() это привязанная к определенному типу
        * функция (статический метод)
        * Эта ф-ция создает новый, пустой экземпляр типа
        * String
        */
        let mut guess = String::new();

        /*
        * Без подключения либы io::stdin() => std::io::stdin()
        * ф-ция возвращает обработчик стандартного ввода
        * .read_line(&mut guess) использует обработчик для
        * получения всего, что введет пользователь и помещает
        * данные в ссылку &mut String переданную как аргумент
        * при неудачном выполнении метод вернет сообщение об
        * ошибке, записанное в .expect()
        */
        io::stdin().read_line(&mut guess)
            .expect("Не удалось прочитать строку");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // {} - указатель места заполнения
        println!("Ваша попытка: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Слишком маленькое!"),
            Ordering::Greater => println!("Слишком большое!"),
            Ordering::Equal => {
                println!("Вы выиграли!");
                break;
            }
        }
    }

    println!("Загаданное число: {}", secret_number);
}
