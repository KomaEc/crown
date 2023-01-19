use ::libc;
extern "C" {
    pub type ldat;
    fn cbreak() -> libc::c_int;
    fn endwin() -> libc::c_int;
    fn has_colors() -> bool;
    fn initscr() -> *mut WINDOW;
    fn init_pair(_: libc::c_short, _: libc::c_short, _: libc::c_short) -> libc::c_int;
    fn intrflush(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn keypad(_: *mut WINDOW, _: bool) -> libc::c_int;
    fn mvprintw(
        _: libc::c_int,
        _: libc::c_int,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn noecho() -> libc::c_int;
    fn nonl() -> libc::c_int;
    fn printw(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn start_color() -> libc::c_int;
    fn waddch(_: *mut WINDOW, _: chtype) -> libc::c_int;
    fn waddnstr(_: *mut WINDOW, _: *const libc::c_char, _: libc::c_int) -> libc::c_int;
    fn wattrset(_: *mut WINDOW, _: libc::c_int) -> libc::c_int;
    fn wattr_get(
        _: *mut WINDOW,
        _: *mut attr_t,
        _: *mut libc::c_short,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn wclear(_: *mut WINDOW) -> libc::c_int;
    fn wclrtoeol(_: *mut WINDOW) -> libc::c_int;
    fn wgetch(_: *mut WINDOW) -> libc::c_int;
    fn wmove(_: *mut WINDOW, _: libc::c_int, _: libc::c_int) -> libc::c_int;
    fn wrefresh(_: *mut WINDOW) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    static mut curscr: *mut WINDOW;
    static mut stdscr: *mut WINDOW;
    static mut COLS: libc::c_int;
    static mut LINES: libc::c_int;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn sleep(__seconds: libc::c_uint) -> libc::c_uint;
}
pub type __time_t = libc::c_long;
pub type chtype = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _win_st {
    pub _cury: libc::c_short,
    pub _curx: libc::c_short,
    pub _maxy: libc::c_short,
    pub _maxx: libc::c_short,
    pub _begy: libc::c_short,
    pub _begx: libc::c_short,
    pub _flags: libc::c_short,
    pub _attrs: attr_t,
    pub _bkgd: chtype,
    pub _notimeout: bool,
    pub _clear: bool,
    pub _leaveok: bool,
    pub _scroll: bool,
    pub _idlok: bool,
    pub _idcok: bool,
    pub _immed: bool,
    pub _sync: bool,
    pub _use_keypad: bool,
    pub _delay: libc::c_int,
    pub _line: *mut ldat,
    pub _regtop: libc::c_short,
    pub _regbottom: libc::c_short,
    pub _parx: libc::c_int,
    pub _pary: libc::c_int,
    pub _parent: *mut WINDOW,
    pub _pad: pdat,
    pub _yoffset: libc::c_short,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdat {
    pub _pad_y: libc::c_short,
    pub _pad_x: libc::c_short,
    pub _pad_top: libc::c_short,
    pub _pad_left: libc::c_short,
    pub _pad_bottom: libc::c_short,
    pub _pad_right: libc::c_short,
}
pub type WINDOW = _win_st;
pub type attr_t = chtype;
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_object {
    pub x: libc::c_int,
    pub y: libc::c_int,
    pub color: libc::c_int,
    pub bold: bool,
    pub character: libc::c_char,
}
static mut ver: *mut libc::c_char = b"1.7320508.406\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut messages: [*mut libc::c_char; 406] = [
    b"\"I pity the fool who mistakes me for kitten!\", sez Mr. T.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"That's just an old tin can.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's an altar to the horse god.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A box of dancing mechanical pencils. They dance! They sing!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's an old Duke Ellington record.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A box of fumigation pellets.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A digital clock. It's stuck at 2:17 PM.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"That's just a charred human corpse.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"I don't know what that is, but it's not kitten.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"An empty shopping bag. Paper or plastic?\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Could it be... a big ugly bowling trophy?\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A coat hanger hovers in thin air. Odd.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Not kitten, just a packet of Kool-Aid(tm).\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A freshly-baked pumpkin pie.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A lone, forgotten comma, sits here, sobbing.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"ONE HUNDRED THOUSAND CARPET FIBERS!!!!!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's Richard Nixon's nose!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's Lucy Ricardo. \"Aaaah, Ricky!\", she says.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"You stumble upon Bill Gates' stand-up act.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Just an autographed copy of the Kama Sutra.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the Will Rogers Highway. Who was Will Rogers, anyway?\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's another robot, more advanced in design than you but strangely immobile.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Leonard Richardson is here, asking people to lick him.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a stupid mask, fashioned after a beagle.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Your State Farm Insurance(tm) representative!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's the local draft board.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Seven 1/4\" screws and a piece of plastic.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"An 80286 machine.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"One of those stupid \"Homes of the Stars\" maps.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A signpost saying \"TO KITTEN\". It points in no particular direction.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A hammock stretched between a tree and a volleyball pole.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A Texas Instruments of Destruction calculator.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a dark, amphorous blob of matter.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Just a pincushion.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's a mighty zombie talking about some love and prosperity.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"Dear robot, you may have already won our 10 MILLION DOLLAR prize...\"\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's just an object.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A mere collection of pixels.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A badly dented high-hat cymbal lies on its side here.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A marijuana brownie.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A plush Chewbacca.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Daily hunger conditioner from Australasia\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Just some stuff.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Why are you touching this when you should be finding kitten?\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A glorious fan of peacock feathers.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's some compromising photos of Babar the Elephant.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A copy of the Weekly World News. Watch out for the chambered nautilus!\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's the proverbial wet blanket.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A \"Get Out of Jail Free\" card.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"An incredibly expensive \"Mad About You\" collector plate.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Paul Moyer's necktie.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A haircut and a real job. Now you know where to get one!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"An automated robot-hater. It frowns disapprovingly at you.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"An automated robot-liker. It smiles at you.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a black hole. Don't fall in!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Just a big brick wall.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"You found kitten! No, just kidding.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Heart of Darkness brand pistachio nuts.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A smoking branding iron shaped like a 24-pin connector.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a Java applet.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"An abandoned used-car lot.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A shameless plug for Crummy: http://www.crummy.com/\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A shameless plug for the UCLA Linux Users Group: http://linux.ucla.edu/\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A can of Spam Lite.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This is another fine mess you've gotten us into, Stanley.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's scenery for \"Waiting for Godot\".\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This grain elevator towers high above you.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A Mentos wrapper.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's the constellation Pisces.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a fly on the wall. Hi, fly!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This kind of looks like kitten, but it's not.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a banana! Oh, joy!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A helicopter has crashed here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Carlos Tarango stands here, doing his best impression of Pat Smear.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A patch of mushrooms grows here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A patch of grape jelly grows here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A spindle, and a grindle, and a bucka-wacka-woom!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A geyser sprays water high into the air.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A toenail? What good is a toenail?\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You've found the fish! Not that it does you much good in this game.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A Buttertonsils bar.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"One of the few remaining discoes.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Ah, the uniform of a Revolutionary-era minuteman.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A punch bowl, filled with punch and lemon slices.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's nothing but a G-thang, baby.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"IT'S ALIVE! AH HA HA HA HA!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This was no boating accident!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Wait! This isn't the poker chip! You've been tricked! DAMN YOU, MENDEZ!\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A livery stable! Get your livery!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a perpetual immobility machine.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\"On this spot in 1962, Henry Winkler was sick.\"\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"There's nothing here; it's just an optical illusion.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The World's Biggest Motzah Ball!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A tribe of cannibals lives here. They eat Malt-O-Meal for breakfast, you know.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This appears to be a rather large stack of trashy romance novels.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Look out! Exclamation points!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A herd of wild coffee mugs slumbers here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a limbo bar! How low can you go?\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the horizon. Now THAT'S weird.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A vase full of artificial flowers is stuck to the floor here.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A large snake bars your way.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A pair of saloon-style doors swing slowly back and forth here.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's an ordinary bust of Beethoven... but why is it painted green?\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's TV's lovable wisecracking Crow! \"Bite me!\", he says.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Hey, look, it's war. What is it good for? Absolutely nothing. Say it again.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's the amazing self-referential thing that's not kitten.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A flamboyant feather boa. Now you can dress up like Carol Channing!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"Sure hope we get some rain soon,\" says Farmer Joe.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"How in heck can I wash my neck if it ain't gonna rain no more?\" asks Farmer Al.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\"Topsoil's all gone, ma,\" weeps Lil' Greg.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This is a large brown bear. Oddly enough, it's currently peeing in the woods.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A team of arctic explorers is camped here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This object here appears to be Louis Farrakhan's bow tie.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This is the world-famous Chain of Jockstraps.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A trash compactor, compacting away.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This toaster strudel is riddled with bullet holes!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a hologram of a crashed helicopter.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This is a television. On screen you see a robot strangely similar to yourself.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This balogna has a first name, it's R-A-N-C-I-D.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A salmon hatchery? Look again. It's merely a single salmon.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a rim shot. Ba-da-boom!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's creepy and it's kooky, mysterious and spooky. It's also somewhat ooky.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This is an anagram.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This object is like an analogy.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a symbol. You see in it a model for all symbols everywhere.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The object pushes back at you.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A traffic signal. It appears to have been recently vandalized.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"There is no kitten!\" cackles the old crone. You are shocked by her blasphemy.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This is a Lagrange point. Don't come too close now.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The dirty old tramp bemoans the loss of his harmonica.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Look, it's Fanny the Irishman!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"What in blazes is this?\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the instruction manual for a previous version of this game.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A brain cell. Oddly enough, it seems to be functioning.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Tea and/or crumpets.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This jukebox has nothing but Cliff Richards albums in it.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a Quaker Oatmeal tube, converted into a drum.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This is a remote control. Being a robot, you keep a wide berth.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a roll of industrial-strength copper wire.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Oh boy! Grub! Er, grubs.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A puddle of mud, where the mudskippers play.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Plenty of nothing.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Look at that, it's the Crudmobile.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Just Walter Mattheau and Jack Lemmon.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Two crepes, two crepes in a box.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"An autographed copy of \"Primary Colors\", by Anonymous.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Another rabbit? That's three today!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a segmentation fault. Core dumped, by the way.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A historical marker showing the actual location of /dev/null.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Thar's Mobius Dick, the convoluted whale. Arrr!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a charcoal briquette, smoking away.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A pizza, melting in the sun.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a \"HOME ALONE 2: Lost in New York\" novelty cup.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A stack of 7 inch floppies wobbles precariously.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's nothing but a corrupted floppy. Coaster anyone?\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A section of glowing phosphor cells sings a song of radiation to you.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This TRS-80 III is eerily silent.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A toilet bowl occupies this space.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This peg-leg is stuck in a knothole!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a solitary vacuum tube.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This corroded robot is clutching a mitten.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\"Hi, I'm Anson Williams, TV's 'Potsy'.\"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This subwoofer was blown out in 1974.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Three half-pennies and a wooden nickel.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the missing chapter to \"A Clockwork Orange\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a burrito stand flyer. \"Taqueria El Ranchito\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This smiling family is happy because they eat LARD.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Roger Avery, persona un famoso de los Estados Unidos.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Ne'er but a potted plant.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A parrot, kipping on its back.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A forgotten telephone switchboard.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A forgotten telephone switchboard operator.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's an automated robot-disdainer. It pretends you're not there.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a portable hole. A sign reads: \"Closed for the winter\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Just a moldy loaf of bread.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A little glass tub of Carmex. ($.89) Too bad you have no lips.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A Swiss-Army knife. All of its appendages are out. (toothpick lost)\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a zen simulation, trapped within an ASCII character.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a copy of \"The Rubaiyat of Spike Schudy\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's \"War and Peace\" (unabridged, very small print).\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A willing, ripe tomato bemoans your inability to digest fruit.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A robot comedian. You feel amused.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's KITT, the talking car.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Here's Pete Peterson. His batteries seem to have long gone dead.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"Blup, blup, blup\", says the mud pot.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"More grist for the mill.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Grind 'em up, spit 'em out, they're twigs.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"The boom box cranks out an old Ethel Merman tune.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's \"Finding kitten\", published by O'Reilly and Associates.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Pumpkin pie spice.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's the Bass-Matic '76! Mmm, that's good bass!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"Lend us a fiver 'til Thursday\", pleas Andy Capp.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a tape of '70s rock. All original hits! All original artists!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"You've found the fabled America Online disk graveyard!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Empty jewelboxes litter the landscape.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the astounding meta-object.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Ed McMahon stands here, lost in thought. Seeing you, he bellows, \"YES SIR!\"\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"...thingy???\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's 1000 secrets the government doesn't want you to know!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The letters O and R.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A magical... magic thing.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a moment of silence.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's Sirhan-Sirhan, looking guilty.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's \"Chicken Soup for the Kitten-seeking Soulless Robot.\"\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It is a set of wind-up chatter teeth.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It is a cloud shaped like an ox.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You see a snowflake here, melting slowly.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a big block of ice. Something seems to be frozen inside it.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Vladimir Lenin's casket rests here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a copy of \"Zen and The Art of Robot Maintenance\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This invisible box contains a pantomime horse.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A mason jar lies here open. It's label reads: \"do not open!\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A train of thought chugs through here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This jar of pickles expired in 1957.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Someone's identity disk lies here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\"Yes!\" says the bit.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\"No!\" says the bit.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A dodecahedron bars your way.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Mr. Hooper is here, surfing.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a big smoking fish.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You have new mail in /var/spool/robot\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Just a monitor with the blue element burnt out.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A pile of coaxial plumbing lies here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a rotten old shoe.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a hundred-dollar bill.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a Dvorak keyboard.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a cardboard box full of 8-tracks.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Just a broken hard drive containg the archives of Nerth Pork.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A broken metronome sits here, it's needle off to one side.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A sign reads: \"Go home!\"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A sign reads: \"No robots allowed!\"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the handheld robotfindskitten game, by Tiger.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This particular monstrosity appears to be ENIAC.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This is a tasty-looking banana creme pie.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A wireframe model of a hot dog rotates in space here.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Just the empty husk of a locust.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You disturb a murder of crows.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a copy of the robotfindskitten EULA.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's Death.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's an autographed copy of \"Secondary Colors,\" by Bob Ross.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It is a marzipan dreadnought that appears to have melted and stuck.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a DVD of \"Crouching Monkey, Hidden Kitten\", region encoded for the moon.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's Kieran Hervold.  Damn dyslexia!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A non-descript box of crackers.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Carbonated Water, High Fructose Corn Syrup, Color, Phosphoric Acid, Flavors, Caffeine.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\"Move along! Nothing to see here!\"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the embalmed corpse of Vladimir Lenin.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A coupon for one free steak-fish at your local family diner.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A set of keys to a 2001 Rolls Royce. Worthless.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A gravestone stands here.  \"Izchak Miller, ascended.\"\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Someone has written \"ad aerarium\" on the ground here.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A large blue eye floats in midair.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This appears to be a statue of Perseus.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"There is an opulent throne here.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a squad of Keystone Kops.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This seems to be junk mail addressed to the finder of the Eye of Larn.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A wondrous and intricate golden amulet.  Too bad you have no neck.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The swampy ground around you seems to stink with disease.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"An animate blob of acid.  Being metallic, you keep well away.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a copy of Knuth with the chapter on kitten-search algorithms torn out.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A crowd of people, and at the center, a popular misconception.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a blind man. When you touch, he exclaims \"It's a kitten prospecting robot!\"\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's a lost wallet. It's owner didn't have pets, so you discard it.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This place is called Antarctica. There is no kitten here.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a mousetrap, baited with soap.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A book with \"Don't Panic\" in large friendly letters across the cover.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A compendium of haiku about metals.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A discredited cosmology, relic of a bygone era.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A hollow voice says \"Plugh\".\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A knight who says \"Either I am an insane knave, or you will find kitten.\"\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A neural net -- maybe it's trying to recognize kitten.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A screwdriver.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A statue of a girl holding a goose like the one in Gottingen, Germany.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A tetradrachm dated \"42 B.C.\"\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A voice booms out \"Onward, kitten soldiers...\"\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"An eminently forgettable zahir.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Apparently, it's Edmund Burke.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"For a moment, you feel something in your hands, but it disappears!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Here is a book about Robert Kennedy.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Hey, robot, leave those lists alone.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Ho hum.  Another synthetic a posteriori.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's Asimov's Laws of Robotics.  You feel a strange affinity for them.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's Bach's Mass in B-minor!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a bug.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's a synthetic a priori truth!  Immanuel would be so pleased!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's the Tiki Room.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Just some old play by a Czech playwright, and you can't read Czech.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Kitten is the letter 'Q'.  Oh, wait, maybe not.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Quidquid Latine dictum sit, kitten non est.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Sutro Tower is visible at some distance through the fog.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The Digital Millennium Copyright Act of 1998.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The United States Court of Appeals for the Federal Circuit.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The non-kitten item like this but with \"false\" and \"true\" switched is true.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"The non-kitten item like this but with \"true\" and \"false\" switched is false.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This is the chapter called \"A Map of the Cat?\" from Feynman's autobiography.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This is the forest primeval.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Werner's \"Pocket Field Guide to Things That Are Not Kitten\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"You found nettik, but that's backwards.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You have found some zinc, but you must not stop here, for you must find kitten.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\"50 Years Among the Non-Kitten Items\", by Ann Droyd.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"Robot may not injure kitten, or, through inaction, ...\"\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"Address Allocation for Private Internets\" by Yakov Rekhter et al.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\"Mail Routing and the Domain System\" by Craig Partridge.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"\"The Theory and Practice of Oligarchical Collectivism\" by Emmanuel Goldstein.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"\"201 Kitten Verbs, Fully Conjugated\".  You look for \"find\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A card shark sits here, practicing his Faro shuffle.  He ignores you.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A copy of DeCSS.  They're a dime a dozen these days.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A demonic voice proclaims \"There is no kitten, only Zuul\".  You flee.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A lotus.  You make an interesting pair.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A milk carton, with a black and white picture of kitten on the side.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Any ordinary robot could see from a mile away that this wasn't kitten.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A stegosaurus, escaped from the stegosaurusfindsrobot game.  It finds you.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Baling wire and chewing gum.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Chewing gum and baling wire.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Here is no kitten but only rock, rock and no kitten and the sandy road.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Hey, I bet you thought this was kitten.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It is an ancient mariner, and he stoppeth one of three.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It pleases you to be kind to what appears to be kitten -- but it's not!\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's a blatant plug for Ogg Vorbis, http://www.vorbis.com/\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a business plan for a new startup, kitten.net.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a revised business plan for a new startup, my.kitten.net.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a square.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It seems to be a copy of \"A Tail of Two Kitties\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's the Donation of Constantine!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's this message, nothing more.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Lysine, an essential amino acid.  Well, maybe not for robots.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"No kitten here.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"The score for a Czech composer's \"Kitten-Finding Symphony in C\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This looks like Bradley's \"Appearance and Reality\", but it's really not.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This non-kitten item no verb.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You feel strangely unfulfilled.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You hit the non-kitten item.  The non-kitten item fails to yowl.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"You suddenly yearn for your distant homeland.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"You've found the snows of yesteryear!  So that's where they all went to.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Approaching.  One car.  J.  Followed by.  Two car.  M, M.  In five. Minutes.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Free Jon Johansen!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Free Dmitry Sklyarov!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"One person shouts \"What do we want?\" The crowd answers \"Free Dmitry!\"\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Judith Platt insults librarians.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This map is not the territory.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"\"Go back to Libraria!\", says Pat Schroeder.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"This is a porcelain kitten-counter.  0, 0, 0, 0, 0...\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"An old bootable business card, unfortunately cracked down the middle.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A kitten sink, for washing kitten (if only kitten liked water).\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A kitten source (to match the kitten sink).\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"If it's one thing, it's not another.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"If it's not one thing, it's another.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A caboodle.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A grin.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A hedgehog.  It looks like it knows something important.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"You've found... Oh wait, that's just a cat.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Robot should not be touching that.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Air Guitar!!!  NA na NA na!!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"An aromatherapy candle burns with healing light.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"You find a bright shiny penny.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a free Jon Johansen!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's a free Dmitry Sklyarov!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"The rothe hits!  The rothe hits!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's an Internet chain letter about sodium laureth sulfate.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Ed Witten sits here, pondering string theory.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Something is written here in the dust.  You read: \"rJbotf ndQkttten\".\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"We wish you a merry kitten, and a happy New Year!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Run away!  Run away!\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"You can see right through this copy of Brin's \"Transparent Society\".\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This copy of \"Steal This Book\" has been stolen from a bookstore.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's Roya Naini.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This kit is the fourteenth in a series of kits named with Roman letters.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This is the tenth key you've found so far.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You find a fraud scheme in which loans are used as security for other loans.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's the phrase \"and her\", written in ancient Greek.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's the author of \"Randomness and Mathematical Proof\".\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's the crusty exoskeleton of an arthropod!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's Emporer Shaddam the 4th's planet!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the triangle leg adjacent to an angle divided by the leg opposite it.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's a bottle of nail polish remover.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"You found netkit! Way to go, robot!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's the ASCII Floating Head of Seth David Schoen!\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A frosted pink party-cake, half eaten.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A bitchin' homemade tesla coil.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Conan O'Brian, sans jawbone.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's either a mirror, or another soulless kitten-seeking robot.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Preoccupation with finding kitten prevents you from investigating further.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Fonzie sits here, mumbling incoherently about a shark and a pair of waterskis.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"The ghost of your dance instructor, his face a paper-white mask of evil.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A bag of groceries taken off the shelf before the expiration date.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A book: Feng Shui, Zen: the art of randomly arranging items that are not kitten.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"This might be the fountain of youth, but you'll never know.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"Tigerbot Hesh.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"Stimutacs.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A canister of pressurized whipped cream, sans whipped cream.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"The non-kitten item bites!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"A chain hanging from two posts reminds you of the Gateway Arch.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"A mathematician calculates the halting probability of a Turing machine.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A number of short theatrical productions are indexed 1, 2, 3, ... n.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"A technical university in Australia.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It is -- I just feel something wonderful is about to happen.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
    b"It's a Cat 5 cable.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's a U.S. president.\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"It's a piece of cloth used to cover a stage in between performances.\0"
        as *const u8 as *const libc::c_char as *mut libc::c_char,
    b"The ionosphere seems charged with meaning.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"This tomography is like, hella axial, man!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's your favorite game -- robotfindscatan!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"Just a man selling an albatross.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"The intermission from a 1930s silent movie.\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"It's an inverted billiard ball!\0" as *const u8 as *const libc::c_char
        as *mut libc::c_char,
    b"The spectre of Sherlock Holmes wills you onwards.\0" as *const u8
        as *const libc::c_char as *mut libc::c_char,
];
#[no_mangle]
pub static mut robot: screen_object = screen_object {
    x: 0,
    y: 0,
    color: 0,
    bold: false,
    character: 0,
};
#[no_mangle]
pub static mut kitten: screen_object = screen_object {
    x: 0,
    y: 0,
    color: 0,
    bold: false,
    character: 0,
};
#[no_mangle]
pub static mut num_bogus: libc::c_int = 0;
#[no_mangle]
pub static mut bogus: [screen_object; 406] = [screen_object {
    x: 0,
    y: 0,
    color: 0,
    bold: false,
    character: 0,
}; 406];
#[no_mangle]
pub static mut bogus_messages: [libc::c_int; 406] = [0; 406];
#[no_mangle]
pub static mut used_messages: [libc::c_int; 406] = [0; 406];
#[no_mangle]
pub static mut screen: *mut *mut libc::c_int = 0 as *const *mut libc::c_int
    as *mut *mut libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn full_draw(mut o: screen_object, mut in_place: bool) {
    let mut old: attr_t = 0;
    let mut dummy: libc::c_short = 0;
    let mut new: attr_t = 0;
    wattr_get(stdscr, &mut old, &mut dummy, 0 as *mut libc::c_void);
    new = (o.color as chtype) << 0 as libc::c_int + 8 as libc::c_int
        & ((1 as libc::c_uint) << 8 as libc::c_int).wrapping_sub(1 as libc::c_uint)
            << 0 as libc::c_int + 8 as libc::c_int;
    if o.character as libc::c_int == '#' as i32 {
        new |= (1 as libc::c_uint) << 12 as libc::c_int + 8 as libc::c_int;
    }
    if o.character as libc::c_int <= '\u{1a}' as i32 {
        new |= (1 as libc::c_uint) << 14 as libc::c_int + 8 as libc::c_int;
    }
    if o.bold {
        new |= (1 as libc::c_uint) << 13 as libc::c_int + 8 as libc::c_int;
    }
    wattrset(stdscr, new as libc::c_int);
    if in_place {
        printw(b"%c\0" as *const u8 as *const libc::c_char, o.character as libc::c_int);
    } else {
        mvprintw(
            o.y,
            o.x,
            b"%c\0" as *const u8 as *const libc::c_char,
            o.character as libc::c_int,
        );
        wmove(stdscr, o.y, o.x);
    }
    wattrset(stdscr, old as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn draw(mut o: screen_object) {
    full_draw(o, 0 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn draw_in_place(mut o: screen_object) {
    full_draw(o, 1 as libc::c_int != 0);
}
#[no_mangle]
pub unsafe extern "C" fn message(mut message_0: *mut libc::c_char) {
    wmove(stdscr, 1 as libc::c_int, 0 as libc::c_int);
    wclrtoeol(stdscr);
    mvprintw(
        1 as libc::c_int,
        0 as libc::c_int,
        b"%.*s\0" as *const u8 as *const libc::c_char,
        COLS,
        message_0,
    );
    wmove(stdscr, robot.y, robot.x);
    wrefresh(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn play_game() {
    let mut old_x = robot.x;
    let mut old_y = robot.y;
    let mut input: libc::c_int = 0;
    input = wgetch(stdscr);
    while input != 27 as libc::c_int && input != 'q' as i32 && input != 'Q' as i32 {
        process_input(input);
        if !(old_x == robot.x && old_y == robot.y) {
            if wmove(stdscr, old_y, old_x) == -(1 as libc::c_int) {} else {
                waddch(stdscr, ' ' as i32 as chtype);
            };
            *(*screen.offset(old_x as isize))
                .offset(old_y as isize) = -(1 as libc::c_int);
            draw(robot);
            wrefresh(stdscr);
            *(*screen.offset(robot.x as isize))
                .offset(robot.y as isize) = 0 as libc::c_int;
            old_x = robot.x;
            old_y = robot.y;
        }
        input = wgetch(stdscr);
    }
    message(b"Bye!\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    wrefresh(stdscr);
    finish(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn process_input(mut input: libc::c_int) {
    let mut check_x = robot.x;
    let mut check_y = robot.y;
    match input {
        12 => {
            wrefresh(curscr);
        }
        259 | 107 | 75 | 16 => {
            check_y -= 1;
        }
        262 | 121 | 89 => {
            check_x -= 1;
            check_y -= 1;
        }
        339 | 117 | 85 => {
            check_x += 1;
            check_y -= 1;
        }
        258 | 106 | 74 | 14 => {
            check_y += 1;
        }
        360 | 98 | 66 => {
            check_x -= 1;
            check_y += 1;
        }
        338 | 110 | 78 => {
            check_x += 1;
            check_y += 1;
        }
        260 | 104 | 72 | 2 => {
            check_x -= 1;
        }
        261 | 108 | 76 | 6 => {
            check_x += 1;
        }
        0 => {}
        _ => {
            message(
                b"Invalid input: Use direction keys or Esc.\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            return;
        }
    }
    if check_y < 3 as libc::c_int || check_y > LINES - 1 as libc::c_int
        || check_x < 0 as libc::c_int || check_x > COLS - 1 as libc::c_int
    {
        return;
    }
    if *(*screen.offset(check_x as isize)).offset(check_y as isize)
        != -(1 as libc::c_int)
    {
        match *(*screen.offset(check_x as isize)).offset(check_y as isize) {
            0 => {}
            1 => {
                wmove(stdscr, 1 as libc::c_int, 0 as libc::c_int);
                wclrtoeol(stdscr);
                play_animation(input);
            }
            _ => {
                message(
                    messages[bogus_messages[(*(*screen.offset(check_x as isize))
                        .offset(check_y as isize) - 2 as libc::c_int) as usize] as usize],
                );
            }
        }
        return;
    }
    robot.x = check_x;
    robot.y = check_y;
}
unsafe extern "C" fn finish(mut sig: libc::c_int) {
    endwin();
    printf(
        b"%c%c%c\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '(' as i32,
        'B' as i32,
    );
    exit(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn validchar(mut a: libc::c_char) -> libc::c_int {
    match a as libc::c_int {
        35 | 32 | 127 => return 0 as libc::c_int,
        _ => {}
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn play_animation(mut input: libc::c_int) {
    let mut counter: libc::c_int = 0;
    counter = 4 as libc::c_int;
    while counter > 0 as libc::c_int {
        if wmove(
            stdscr,
            1 as libc::c_int,
            50 as libc::c_int + counter + 1 as libc::c_int,
        ) == -(1 as libc::c_int)
        {} else {
            waddch(stdscr, ' ' as i32 as chtype);
        };
        wmove(stdscr, 1 as libc::c_int, 50 as libc::c_int + counter);
        if input == 0o405 as libc::c_int || input == 0o402 as libc::c_int
            || input == 0o540 as libc::c_int || input == 0o535 as libc::c_int
        {
            draw_in_place(kitten);
        } else {
            draw_in_place(robot);
        }
        if wmove(stdscr, 1 as libc::c_int, 50 as libc::c_int - counter)
            == -(1 as libc::c_int)
        {} else {
            waddch(stdscr, ' ' as i32 as chtype);
        };
        wmove(stdscr, 1 as libc::c_int, 50 as libc::c_int - counter + 1 as libc::c_int);
        if input == 0o405 as libc::c_int || input == 0o402 as libc::c_int
            || input == 0o540 as libc::c_int || input == 0o535 as libc::c_int
        {
            draw_in_place(robot);
        } else {
            draw_in_place(kitten);
        }
        wrefresh(stdscr);
        sleep(1 as libc::c_int as libc::c_uint);
        counter -= 1;
    }
    wmove(stdscr, 1 as libc::c_int, 0 as libc::c_int);
    waddnstr(
        stdscr,
        b"You found kitten! Way to go, robot!\0" as *const u8 as *const libc::c_char,
        -(1 as libc::c_int),
    );
    wrefresh(stdscr);
    finish(0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn instructions() {
    let mut dummy: libc::c_char = 0;
    mvprintw(
        0 as libc::c_int,
        0 as libc::c_int,
        b"robotfindskitten v%s\n\0" as *const u8 as *const libc::c_char,
        ver,
    );
    printw(
        b"By the illustrious Leonard Richardson (C) 1997, 2000\n\0" as *const u8
            as *const libc::c_char,
    );
    printw(
        b"Written originally for the Nerth Pork robotfindskitten contest\n\n\0"
            as *const u8 as *const libc::c_char,
    );
    printw(b"In this game, you are robot (\0" as *const u8 as *const libc::c_char);
    draw_in_place(robot);
    printw(
        b"). Your job is to find kitten. This task\n\0" as *const u8
            as *const libc::c_char,
    );
    printw(
        b"is complicated by the existence of various things which are not kitten.\n\0"
            as *const u8 as *const libc::c_char,
    );
    printw(
        b"Robot must touch items to determine if they are kitten or not. The game\n\0"
            as *const u8 as *const libc::c_char,
    );
    printw(
        b"ends when robotfindskitten. Alternatively, you may end the game by hitting\n\0"
            as *const u8 as *const libc::c_char,
    );
    printw(
        b"the Esc key. See the documentation for more information.\n\n\0" as *const u8
            as *const libc::c_char,
    );
    printw(b"Press any key to start.\n\0" as *const u8 as *const libc::c_char);
    wrefresh(stdscr);
    dummy = wgetch(stdscr) as libc::c_char;
    wclear(stdscr);
}
#[no_mangle]
pub unsafe extern "C" fn initialize_arrays() {
    let mut counter: libc::c_int = 0;
    let mut counter2: libc::c_int = 0;
    let mut empty = screen_object {
        x: 0,
        y: 0,
        color: 0,
        bold: false,
        character: 0,
    };
    let mut i = 0 as libc::c_int;
    screen = malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul((COLS - 1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong),
    ) as *mut *mut libc::c_int;
    i = 0 as libc::c_int;
    while i < COLS - 1 as libc::c_int + 1 as libc::c_int {
        let ref mut fresh0 = *screen.offset(i as isize);
        *fresh0 = malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(
                    (LINES - 1 as libc::c_int + 1 as libc::c_int) as libc::c_ulong,
                ),
        ) as *mut libc::c_int;
        i += 1;
    }
    empty.x = -(1 as libc::c_int);
    empty.y = -(1 as libc::c_int);
    empty.color = 0 as libc::c_int;
    empty.bold = 0 as libc::c_int != 0;
    empty.character = ' ' as i32 as libc::c_char;
    counter = 0 as libc::c_int;
    while counter <= COLS - 1 as libc::c_int {
        counter2 = 0 as libc::c_int;
        while counter2 <= LINES - 1 as libc::c_int {
            *(*screen.offset(counter as isize))
                .offset(counter2 as isize) = -(1 as libc::c_int);
            counter2 += 1;
        }
        counter += 1;
    }
    counter = 0 as libc::c_int;
    while (counter as libc::c_ulong)
        < (::std::mem::size_of::<[*mut libc::c_char; 406]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
    {
        used_messages[counter as usize] = 0 as libc::c_int;
        bogus_messages[counter as usize] = 0 as libc::c_int;
        bogus[counter as usize] = empty;
        counter += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn initialize_ncurses() {
    signal(2 as libc::c_int, Some(finish as unsafe extern "C" fn(libc::c_int) -> ()));
    initscr();
    keypad(stdscr, 1 as libc::c_int != 0);
    nonl();
    intrflush(stdscr, 0 as libc::c_int != 0);
    noecho();
    cbreak();
    if has_colors() {
        start_color();
        init_pair(
            0 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
        );
        init_pair(
            2 as libc::c_int as libc::c_short,
            2 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
        );
        init_pair(
            1 as libc::c_int as libc::c_short,
            1 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
        );
        init_pair(
            6 as libc::c_int as libc::c_short,
            6 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
        );
        init_pair(
            7 as libc::c_int as libc::c_short,
            7 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
        );
        init_pair(
            5 as libc::c_int as libc::c_short,
            5 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
        );
        init_pair(
            4 as libc::c_int as libc::c_short,
            4 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
        );
        init_pair(
            3 as libc::c_int as libc::c_short,
            3 as libc::c_int as libc::c_short,
            0 as libc::c_int as libc::c_short,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn initialize_robot() {
    robot.x = rand() % (COLS - 1 as libc::c_int) + 1 as libc::c_int;
    robot
        .y = rand() % (LINES - 1 as libc::c_int - 3 as libc::c_int + 1 as libc::c_int)
        + 3 as libc::c_int;
    robot.character = '#' as i32 as libc::c_char;
    robot.color = 0 as libc::c_int;
    robot.bold = 0 as libc::c_int != 0;
    *(*screen.offset(robot.x as isize)).offset(robot.y as isize) = 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn initialize_kitten() {
    loop {
        kitten.x = rand() % (COLS - 1 as libc::c_int) + 1 as libc::c_int;
        kitten
            .y = rand()
            % (LINES - 1 as libc::c_int - 3 as libc::c_int + 1 as libc::c_int)
            + 3 as libc::c_int;
        if !(*(*screen.offset(kitten.x as isize)).offset(kitten.y as isize)
            != -(1 as libc::c_int))
        {
            break;
        }
    }
    loop {
        kitten
            .character = (rand() % (126 as libc::c_int - '!' as i32 + 1 as libc::c_int)
            + '!' as i32) as libc::c_char;
        if !(validchar(kitten.character) == 0) {
            break;
        }
    }
    *(*screen.offset(kitten.x as isize)).offset(kitten.y as isize) = 1 as libc::c_int;
    kitten.color = rand() % 6 as libc::c_int + 1 as libc::c_int;
    kitten
        .bold = if rand() % 2 as libc::c_int != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
#[no_mangle]
pub unsafe extern "C" fn initialize_bogus() {
    let mut counter: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    counter = 0 as libc::c_int;
    while counter < num_bogus {
        bogus[counter as usize].color = rand() % 6 as libc::c_int + 1 as libc::c_int;
        bogus[counter as usize]
            .bold = if rand() % 2 as libc::c_int != 0 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        } != 0;
        loop {
            bogus[counter as usize]
                .character = (rand()
                % (126 as libc::c_int - '!' as i32 + 1 as libc::c_int) + '!' as i32)
                as libc::c_char;
            if !(validchar(bogus[counter as usize].character) == 0) {
                break;
            }
        }
        loop {
            bogus[counter as usize]
                .x = rand() % (COLS - 1 as libc::c_int) + 1 as libc::c_int;
            bogus[counter as usize]
                .y = rand()
                % (LINES - 1 as libc::c_int - 3 as libc::c_int + 1 as libc::c_int)
                + 3 as libc::c_int;
            if !(*(*screen.offset(bogus[counter as usize].x as isize))
                .offset(bogus[counter as usize].y as isize) != -(1 as libc::c_int))
            {
                break;
            }
        }
        *(*screen.offset(bogus[counter as usize].x as isize))
            .offset(bogus[counter as usize].y as isize) = counter + 2 as libc::c_int;
        loop {
            index = (rand() as libc::c_ulong)
                .wrapping_rem(
                    (::std::mem::size_of::<[*mut libc::c_char; 406]>() as libc::c_ulong)
                        .wrapping_div(
                            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                        ),
                ) as libc::c_int;
            if !(used_messages[index as usize] != 0 as libc::c_int) {
                break;
            }
        }
        bogus_messages[counter as usize] = index;
        used_messages[index as usize] = 1 as libc::c_int;
        counter += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn initialize_screen() {
    let mut counter: libc::c_int = 0;
    mvprintw(
        0 as libc::c_int,
        0 as libc::c_int,
        b"robotfindskitten v%s\n\n\0" as *const u8 as *const libc::c_char,
        ver,
    );
    counter = 0 as libc::c_int;
    while counter <= COLS - 1 as libc::c_int {
        printw(b"%c\0" as *const u8 as *const libc::c_char, 95 as libc::c_int);
        counter += 1;
    }
    counter = 0 as libc::c_int;
    while counter < num_bogus {
        draw(bogus[counter as usize]);
        counter += 1;
    }
    draw(kitten);
    draw(robot);
    wrefresh(stdscr);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    if argc == 1 as libc::c_int {
        num_bogus = 20 as libc::c_int;
    } else {
        num_bogus = atoi(*argv.offset(1 as libc::c_int as isize));
        if num_bogus < 0 as libc::c_int
            || num_bogus as libc::c_ulong
                > (::std::mem::size_of::<[*mut libc::c_char; 406]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    )
        {
            printf(
                b"Run-time parameter must be between 0 and %d.\n\0" as *const u8
                    as *const libc::c_char,
                (::std::mem::size_of::<[*mut libc::c_char; 406]>() as libc::c_ulong)
                    .wrapping_div(
                        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ),
            );
            exit(0 as libc::c_int);
        }
    }
    srand(time(0 as *mut time_t) as libc::c_uint);
    printf(
        b"%c%c%c\0" as *const u8 as *const libc::c_char,
        27 as libc::c_int,
        '(' as i32,
        'U' as i32,
    );
    initialize_ncurses();
    initialize_arrays();
    initialize_robot();
    initialize_kitten();
    initialize_bogus();
    instructions();
    initialize_screen();
    play_game();
    return 0;
}
// pub fn main() {
//     let mut args: Vec::<*mut libc::c_char> = Vec::new();
//     for arg in ::std::env::args() {
//         args.push(
//             (::std::ffi::CString::new(arg))
//                 .expect("Failed to convert argument into CString.")
//                 .into_raw(),
//         );
//     }
//     args.push(::std::ptr::null_mut());
//     unsafe {
//         ::std::process::exit(
//             main_0(
//                 (args.len() - 1) as libc::c_int,
//                 args.as_mut_ptr() as *mut *mut libc::c_char,
//             ) as i32,
//         )
//     }
// }
