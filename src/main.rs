/*
Консольная версия Tetris на языке Rust.
Для работы с консолью используется crossterm.
*/
use std::{
    io::{self}, time::{Duration, Instant},
};
use crossterm::{ execute,
    cursor::{Hide, Show},
    terminal::{ EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, },
    event::{Event, KeyEvent, KeyCode, poll, read},
};

/* -- Engine -- */
enum Action {
    MoveLeft,
    MoveRight,
    SoftDrop,
    Rotate,
    Drop,    
}

#[derive(Eq, PartialEq)]
enum GameStatus {
    Playing,
    Quit,
    Pause,
}

struct Engine {}
impl Engine {
    fn new()->Self {
        Self{}
    }

    fn handle_action(&self, action: Action) {

    }

    fn update(&mut self) -> GameStatus {
        GameStatus::Playing
    }
}
/* -- Game -- */

// тетрамино
enum TetraType { I, O, Z, S, T, L, J, }

// оригинальные цвета тетрамино
enum Color { Cyan, Yellow, Purple, Green, Red, Blue, Orange,}
// 
enum Cell { Empty, Fill(Color) }

struct Board {
    // вектор строк, каждая строка состоит из нескольких Cell
    cells: Vec<Vec<Cell>>,
}

struct ActiveTetra {
    x: u8,
    y: u8,
    kind: TetraType,
    
}

/* -- Render -- */
#[derive(Copy, Clone)]
struct Render {}
impl Render {
    fn new() -> Self {
        Self{}
    }

    fn draw_frame(self, engine:  &Engine) {

    }
}

/* -- main -- */

// Гард-объект для перевода консоли в режим raw_mode и обратно
struct ConsoleGuard;
impl ConsoleGuard {
    fn new() -> Self {
        let _ = enable_raw_mode();
        let _ = execute!{
            io::stdout(),
            EnterAlternateScreen, 
            Hide,
        };
        // весь вывод теперь направляется в альтернативный экран (буфер)
        // команда Hide скрывает курсор
        Self{}
    }
}
impl Drop for ConsoleGuard {
    fn drop (&mut self) {
        let _ = execute!{
            io::stdout(),
            LeaveAlternateScreen,
            Show,
        };
        let _ = disable_raw_mode();
    }
}

/*
базовый  игровой цикл (gameplay loop)
1. Вызов / новая фигура в стакане
2. Действие / перемещаем, поворачиваем фигуру
3. Результат / фигура упала на дно или на другие фигуры
4. Обратная связь / фиксация фигуры, удаление полных линий
5. Вызов...


технический игровой цикл (technical loop)
выполняется 60 кадров в секунду (или более), состоит из этапов:
1. Обработка ввода
2. Обработка логики
3. Рисование

*/

fn main() {
    println!("Hello, Tetrus!");

    let _guard = ConsoleGuard::new();

    let mut engine = Engine::new();
    let render = Render::new();

    // 1000 мс / 60 кадров = 16.7 мс
    const UPDATE_PERIOD:Duration = Duration::from_millis(16);

    let mut starting = Instant::now();
    let mut period = Duration::from_millis(0);

    loop {
        // если пришло клавиатурное событие - читаем, обрабатываем, двигаем фигуру
        if poll(Duration::from_secs(0)).unwrap() {
            if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
                let action = match code {
                    KeyCode::Left => Action::MoveLeft,
                    KeyCode::Right => Action::MoveRight,
                    KeyCode::Down => Action::SoftDrop,
                    KeyCode::Up => Action::Rotate,
                    KeyCode::Char(' ') => Action::Drop,
                    KeyCode::Esc => break,
                    _ => continue,
                };

                engine.handle_action(action);
            }
        };        

        let elapsed = starting.elapsed();
        starting = Instant::now();
        period += elapsed;
        
        // обновление логики строго раз в кадр
        if period >= UPDATE_PERIOD {
            let game_status = engine.update();
            if game_status == GameStatus::Quit {
                break;
            }
            period -= UPDATE_PERIOD;
        }
        
        // обновление экрана при первой же возможности
        render.draw_frame(&engine);
    }
}
