use std::io;  // Для ввода/вывода (IO)
use rand::Rng;  // Для генерации случайных чисел
fn main() {
    println!("Добро пожаловать в симулятор броска игральных костей!");

    loop {
        // Запрашиваем количество костей для броска
        println!("Сколько костей вы хотите бросить?");
        
        let mut input = String::new();  // Создаём изменяемую строковую переменную для хранения ввода пользователя
        io::stdin().read_line(&mut input)  // Считываем строку ввода
            .expect("Не удалось прочитать строку");  // Выводим сообщение об ошибке, если чтение не удалось
        
        let num_dice: u32 = match input.trim().parse() {  // Удаляем пробелы и преобразуем строку в число
            Ok(num) => num,
            Err(_) => continue,  // Если ввод некорректный, начнём заново
        };

        // Инициализируем генератор случайных чисел
        let mut rng = rand::thread_rng();

        // Бросаем кости и выводим результаты
        println!("Результаты броска:");
        for _ in 0..num_dice {
            let roll = rng.gen_range(1..= 7);  // Генерируем случайное число от 1 до 6 (верхний предел не включён)
            println!("{}", roll);
        }

        // Спрашиваем, хочет ли пользователь сыграть ещё раз
        println!("Хотите сыграть ещё раз? (да/нет)");

        input.clear();  // Очищаем переменную `input`
        io::stdin().read_line(&mut input)  // Считываем новую строку ввода
            .expect("Не удалось прочитать строку");

        if input.trim().eq_ignore_ascii_case("нет") {
            break;  // Выходим из цикла, если пользователь ввёл "нет"
        }
        
    }
}
    
