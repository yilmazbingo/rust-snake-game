// facilitate high-level interactions between wasm modules and JavaScript
use wasm_bindgen::prelude::*;
// by default rust uses [global_allocator] for memory. instead we are using this library
use wee_alloc::WeeAlloc;
// wasm-pack build --target web
#[global_allocator]
static ALLOC: WeeAlloc = WeeAlloc::INIT;

// getting date.now from javascript."" targets the root folder
#[wasm_bindgen(module="/www/utils/rnd.js")]
extern {
    fn rnd(max:usize)->usize;
}

// PartialEq allows us to use comparison operator ==
#[wasm_bindgen]
#[derive(PartialEq)]
pub enum Direction{
    Up,
    Right,
    Down,
    Left
}

#[wasm_bindgen]
#[derive(Clone,Copy)]
pub enum GameStatus{
    Won,
    Lost,
    Played,
}
//The derive attribute allows new items to be automatically generated for data structures.
#[derive(PartialEq,Clone,Copy)]
// this is tuple-like structs
pub struct SnakeCell(usize);
struct Snake{
    body:Vec<SnakeCell>,
    direction:Direction,
}
// The impl keyword in Rust is used to implement some functionality on types. either struct or enumfmat
impl Snake{
    fn new(spawn_index:usize,size:usize)->Snake{
        let mut body=vec!();
        for i in 0..size{
            body.push(SnakeCell(spawn_index-i));
        }
        // Vec gets instantiated with a macro
        Snake { 
            body,
            direction: Direction::Right,
        }
    }
}

#[wasm_bindgen]
pub struct World{
    // this is your os size
    width:usize,
    snake:Snake,
    size:usize,
    // rust does not have null. this is the way to represent None values. 
    // every Option is either Some and contains a value, or None , and does not. 
    next_cell:Option<SnakeCell>,
    reward_cell:Option<usize>,
    status:Option<GameStatus>,
    points:usize,
}

#[wasm_bindgen]
impl World{
    // this is factory function. new is not a keyword
    pub fn new(width:usize, snake_idx:usize)->World{
        let size=width*width;
        let snake=Snake::new(snake_idx,3);
     
        World { 
            width,
             // we cannot use self here
             //first we consumed the reference of &snake.body and then we used `snake:snake`. otherwise it would give error
             reward_cell:World::gen_reward_cell(size,&snake.body),
            snake,
            size:width*width,
            next_cell:None,
            status:None,
            points:0,
           
        }
    }

    pub fn points(&self)->usize{
        self.points
    }

    fn gen_reward_cell(max:usize,snake_body:&Vec<SnakeCell>)->Option<usize>{
        let mut reward_cell;
        // this is an inifite loop. we break if reward cell is not in snake.body
        loop{
            reward_cell=rnd(max);
            if !snake_body.contains(&SnakeCell(reward_cell)){break;}
        }
        return Some(reward_cell);
    }
    // since we passed "&self" this is a method and can be called by the instances
    pub fn width(&self)->usize{
        self.width
    }
    // &self This means you'll be passing in a reference to the object, as opposed to moving the object itself.
    pub fn reward_cell(&self)->Option<usize>{
        self.reward_cell
    }

    pub fn start_game(&mut self){
        self.status=Some(GameStatus::Played);
    }

    //*********w  we need to mark GameStatus as COPY. COPY IS TOGETHER WITH CLONE. you cannot provide copy without clone */
    pub fn game_status(&self)->Option<GameStatus>{
        return self.status;
    }

    pub fn game_status_text(&self) -> String {
        // match is exhaustive. means that we have to match all possible values. 
        match self.status{
            Some(GameStatus::Won)=>String::from("You have won"),
            Some(GameStatus::Lost)=>String::from("You have lost"),
            Some(GameStatus::Played)=>String::from("Playing"),
            None=>String::from("No Status"),
            }
    }

    pub fn snake_head_idx(&self)->usize{
        self.snake.body[0].0
    }
    // if you are using type outside this scope, we have to mark it with #[wasm_bindgen]
// since we are using Direction inside here, we have to go and mark it with #[wasm_bindgen] 
    pub fn change_snake_dir(&mut self,direction:Direction){
        // write a guard to make sure we are allowed to change direction
        // if we are in  right---->  we cannot do in the same row <------. we have to go up or down
        let next_cell=self.gen_next_snake_cell(&direction);
        if self.snake.body[1].0==next_cell.0{
            return;
        }
        // Since Option::Some used too often, rust allows us using this syntax
        self.next_cell=Some(next_cell);
        self.snake.direction=direction;
    }

    //**********************IMPORTANT BORROWING REFERENCE TO JS ************** */
    /* 
        pub fn snake_cells(&self)->Vec<SnakeCell>{
            // when we return this, this will be out of "self" and it will be destroyed. we wont have access to self. so self.update will be undefined
            // we cannot return references from rust to js. Imagine snake_body would still exist, but you would still have reference in index.ts and you would using the wrong address in the memory you would access the wrong data. 
            self.snake.body }
    */
    // cannot return a reference to JS because of borrowing rules. this means, we do not have control overthe resource snake_body
    // we can tell compiler that we are sure, we want a reference of this spot in the memory, we like to access the reference and we are not afraid of borrowing. Because we know snake_body exists entire duration of our application. 
    // *const is raw operator. Borrowing rule does not apply. raw pointers are not safe. they are heavily used when u are creating this interoperation between different languages
    pub fn snake_cells(&self)->*const SnakeCell{
        // as_ptr is the reference to the first item in the vector
        self.snake.body.as_ptr()
    }
    fn main(){
        let mut message=String::from("Hello");
        let message_2=&message;
        message.push_str(" World");
    }
    

    pub fn snake_length(&self)->usize{
        self.snake.body.len()
    }

    // if you want to assign a new value we write mut
    pub fn step(&mut self){
        match self.status{
            Some(GameStatus::Played)=>{
                // body is vector
            let temp=self.snake.body.clone();
            match self.next_cell{
                Some(cell)=>{
                    self.snake.body[0]=cell;
                    self.next_cell=None;
                },
                None=>{
                    self.snake.body[0]=self.gen_next_snake_cell(&self.snake.direction);
                }
            }
            // we could use self.snake_length
            let len=self.snake.body.len();
            // we do not start from 0 because we already handled moving the head
            for i in 1..len{
                self.snake.body[i]=SnakeCell(temp[i-1].0)
            }
            // ********  check if head touches the end ***********
            if self.snake.body[1..self.snake_length()].contains(&self.snake.body[0]){
                self.status=Some(GameStatus::Lost)
            }
            if self.reward_cell==Some(self.snake_head_idx()){
                if(self.snake_length()<self.size){
                    self.points+=1;
                    self.reward_cell=World::gen_reward_cell(self.size, &self.snake.body);
                }else{
                    // if we have collected everything reward cell would be None
                    self.reward_cell=None;
                    self.status=Some(GameStatus::Won)
                }
                self.snake.body.push(SnakeCell(self.snake.body[1].0));
            }

            },
            // placeholder used for other states
            _=>{}
        }
        
    }

    // ***** MODULO AND DIVISION ARE EXPENSIVE OPERATIONS *****
    fn gen_next_snake_cell(&self, direction:&Direction)->SnakeCell{
        let snake_idx=self.snake_head_idx();
        let row=snake_idx/self.width;
       // when snake cell reaches the ends, it should exit on the other side
        return match direction{
            Direction::Right=>{
                let threshold=(row+1)*self.width();
                if snake_idx+1==threshold{
                    SnakeCell(threshold-self.width)
                } else{
                    SnakeCell(snake_idx+1)
                }
            }
            Direction::Left=>{
                let threshold=row*self.width();
                if snake_idx==threshold{
                    SnakeCell(threshold+(self.width-1))
                } else{
                    SnakeCell(snake_idx-1)
                }

            }
            Direction::Up=>{
                let threshold=snake_idx-(row*self.width);
                if snake_idx==threshold{
                    SnakeCell((self.size-self.width)+threshold)
                } else{
                    SnakeCell(snake_idx-self.width())
                }
            }
            Direction::Down=>{
                let threshold=snake_idx+((self.width-row)*self.width);
                if snake_idx+self.width==threshold{
                    SnakeCell(threshold-((row+1)*self.width))
                } else{
                    SnakeCell(snake_idx+self.width())
                }
            }
        };
           
    }
}

