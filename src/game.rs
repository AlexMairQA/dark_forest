
#[derive(PartialEq)]
enum Woodland
{
TREE,
BRAMBLES,
GROUND
}
type W= Woodland;

struct Position
{
x:usize,
y:usize
}
pub fn play_game()
{
let mut p=Position{x:0, y:0};
use simple_user_input::get_input;
let mut forest=[[W::GROUND, W::GROUND,W::GROUND],
				[W::TREE,W::GROUND, W::TREE],
				[W::BRAMBLES,W::GROUND,W::BRAMBLES],
				[W::GROUND, W::GROUND,W::GROUND]];
				let mut direction:String;
				
				loop
				{
				match forest[p.x][p.y]
				{
				W::TREE
				=>println!("tree"),
				W::BRAMBLES
				=>println!("ouch! brambles!"),
				W::GROUND
				=>println!("ground")
				};
				direction=get_input("Which way do you want to go?");
				
				match direction.as_str()
				{
				"north"=>
				{
				if forest[p.x+1][p.y]==W::TREE
				{
				println!("There is a tree in front of you");
				}
				else
				{
				p.x+=1;
				
				}
				}
				"south"=>
				{
				if forest[p.x-1][p.y]==W::TREE
				{
				println!("There is a tree in front of you");
				}
				else
				{
				p.x-=1;
				
				}
				}
				"east"=>
				{
				if forest[p.x][p.y+1]==W::TREE
				{
				println!("There is a tree in front of you");
				}
				else
				{
				p.y+=1;
				
				}
				}
				"west"=>
				{
				if forest[p.x][p.y-1]==W::TREE
				{
				println!("There is a tree in front of you");
				}
				else
				{
				p.y-=1;
				
				}
				}
				_=>
				println!("{} is not a valid direction", direction),
				};
				}
				}