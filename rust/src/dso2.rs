extern {
    static mut advs_ : Struct3;
    static aindex_ : Struct5;
    static mut cevent_ : Struct9;
    static cindex_ : Struct10;
    static mut findex_ : Struct7;
    fn more_output(arg1 : *const u8);
    fn newsta_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    );
    static mut objcts_ : Struct6;
    static oindex_ : Struct4;
    static mut play_ : Struct2;
    fn printf(__format : *const u8, ...) -> i32;
    static mut rooms_ : Struct1;
    fn rspeak_(arg1 : i32);
    fn rspsub_(arg1 : i32, arg2 : i32);
    static mut state_ : Struct8;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub winner : i32,
    pub here : i32,
    pub telflg : i32,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub garli : i32,
    pub food : i32,
    pub gunk : i32,
    pub coal : i32,
    pub machi : i32,
    pub diamo : i32,
    pub tcase : i32,
    pub bottl : i32,
    pub water : i32,
    pub rope : i32,
    pub knife : i32,
    pub sword : i32,
    pub lamp : i32,
    pub blamp : i32,
    pub rug : i32,
    pub leave : i32,
    pub troll : i32,
    pub axe : i32,
    pub rknif : i32,
    pub keys : i32,
    pub ice : i32,
    pub bar : i32,
    pub coffi : i32,
    pub torch : i32,
    pub tbask : i32,
    pub fbask : i32,
    pub irbox : i32,
    pub ghost : i32,
    pub trunk : i32,
    pub bell : i32,
    pub book : i32,
    pub candl : i32,
    pub match_ : i32,
    pub tube : i32,
    pub putty : i32,
    pub wrenc : i32,
    pub screw : i32,
    pub cyclo : i32,
    pub chali : i32,
    pub thief : i32,
    pub still : i32,
    pub windo : i32,
    pub grate : i32,
    pub door : i32,
    pub hpole : i32,
    pub leak : i32,
    pub rbutt : i32,
    pub raili : i32,
    pub pot : i32,
    pub statu : i32,
    pub iboat : i32,
    pub dboat : i32,
    pub pump : i32,
    pub rboat : i32,
    pub stick : i32,
    pub buoy : i32,
    pub shove : i32,
    pub ballo : i32,
    pub recep : i32,
    pub guano : i32,
    pub brope : i32,
    pub hook1 : i32,
    pub hook2 : i32,
    pub safe : i32,
    pub sslot : i32,
    pub brick : i32,
    pub fuse : i32,
    pub gnome : i32,
    pub blabe : i32,
    pub dball : i32,
    pub tomb : i32,
    pub lcase : i32,
    pub cage : i32,
    pub rcage : i32,
    pub spher : i32,
    pub sqbut : i32,
    pub flask : i32,
    pub pool : i32,
    pub saffr : i32,
    pub bucke : i32,
    pub ecake : i32,
    pub orice : i32,
    pub rdice : i32,
    pub blice : i32,
    pub robot : i32,
    pub ftree : i32,
    pub bills : i32,
    pub portr : i32,
    pub scol : i32,
    pub zgnom : i32,
    pub egg : i32,
    pub begg : i32,
    pub baubl : i32,
    pub canar : i32,
    pub bcana : i32,
    pub ylwal : i32,
    pub rdwal : i32,
    pub pindr : i32,
    pub rbeam : i32,
    pub odoor : i32,
    pub qdoor : i32,
    pub cdoor : i32,
    pub num1 : i32,
    pub num8 : i32,
    pub warni : i32,
    pub cslit : i32,
    pub gcard : i32,
    pub stldr : i32,
    pub hands : i32,
    pub wall : i32,
    pub lungs : i32,
    pub sailo : i32,
    pub aviat : i32,
    pub teeth : i32,
    pub itobj : i32,
    pub every : i32,
    pub valua : i32,
    pub oplay : i32,
    pub wnort : i32,
    pub gwate : i32,
    pub master : i32,
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub player : i32,
    pub arobot : i32,
    pub amastr : i32,
}

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
    pub olnt : i32,
    pub odesc1 : [i32; 220],
    pub odesc2 : [i32; 220],
    pub odesco : [i32; 220],
    pub oactio : [i32; 220],
    pub oflag1 : [i32; 220],
    pub oflag2 : [i32; 220],
    pub ofval : [i32; 220],
    pub otval : [i32; 220],
    pub osize : [i32; 220],
    pub ocapac : [i32; 220],
    pub oroom : [i32; 220],
    pub oadv : [i32; 220],
    pub ocan : [i32; 220],
    pub oread : [i32; 220],
}

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn moveto_(mut nr : i32, mut who : i32) -> i32 {
    let mut ret_val : i32;
    let mut j : i32;
    let mut lhr : i32;
    let mut lnr : i32;
    let mut nlv : i32;
    let mut bits : i32;
    ret_val = 0i32;
    lhr = (rooms_.rflag[
               (play_.here - 1i32) as (usize)
           ] & 8192i32 != 0i32) as (i32);
    lnr = (rooms_.rflag[
               (nr - 1i32) as (usize)
           ] & 8192i32 != 0i32) as (i32);
    j = advs_.avehic[(who - 1i32) as (usize)];
    if j != 0i32 {
        bits = 0i32;
        if j == oindex_.rboat {
            bits = 4096i32;
        }
        if j == oindex_.ballo {
            bits = 2048i32;
        }
        if j == oindex_.bucke {
            bits = 128i32;
        }
        nlv = (rooms_.rflag[
                   (nr - 1i32) as (usize)
               ] & bits == 0i32) as (i32);
        if lnr == 0 && (nlv != 0) || lnr != 0 && (lhr != 0) && (nlv != 0) && (bits != 8192i32) {
            rspsub_(428i32,objcts_.odesc2[(j - 1i32) as (usize)]);
            return ret_val;
        }
    } else if lnr == 0 {
        rspeak_(427i32);
        return ret_val;
    }
    ret_val = 1i32;
    if rooms_.rflag[(nr - 1i32) as (usize)] & 256i32 == 0i32 {
        if who != aindex_.player {
            newsta_(advs_.aobj[(who - 1i32) as (usize)],0i32,nr,0i32,0i32);
        }
        if j != 0i32 {
            newsta_(j,0i32,nr,0i32,0i32);
        }
        play_.here = nr;
        advs_.aroom[(who - 1i32) as (usize)] = play_.here;
        scrupd_(rooms_.rval[(nr - 1i32) as (usize)]);
        rooms_.rval[(nr - 1i32) as (usize)] = 0i32;
        ret_val
    } else {
        rspeak_(
            *(&mut rooms_ as (*mut Struct1) as (*mut i32)).offset(
                 601isize
             ).offset(
                 (nr - 1i32) as (isize)
             )
        );
        ret_val
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
    pub trollf : i32,
    pub cagesf : i32,
    pub bucktf : i32,
    pub caroff : i32,
    pub carozf : i32,
    pub lwtidf : i32,
    pub domef : i32,
    pub glacrf : i32,
    pub echof : i32,
    pub riddlf : i32,
    pub lldf : i32,
    pub cyclof : i32,
    pub magicf : i32,
    pub litldf : i32,
    pub safef : i32,
    pub gnomef : i32,
    pub gnodrf : i32,
    pub mirrmf : i32,
    pub egyptf : i32,
    pub onpolf : i32,
    pub blabf : i32,
    pub brieff : i32,
    pub superf : i32,
    pub buoyf : i32,
    pub grunlf : i32,
    pub gatef : i32,
    pub rainbf : i32,
    pub cagetf : i32,
    pub empthf : i32,
    pub deflaf : i32,
    pub glacmf : i32,
    pub frobzf : i32,
    pub endgmf : i32,
    pub badlkf : i32,
    pub thfenf : i32,
    pub singsf : i32,
    pub mrpshf : i32,
    pub mropnf : i32,
    pub wdopnf : i32,
    pub mr1f : i32,
    pub mr2f : i32,
    pub inqstf : i32,
    pub follwf : i32,
    pub spellf : i32,
    pub cpoutf : i32,
    pub cpushf : i32,
    pub btief : i32,
    pub binff : i32,
    pub rvmnt : i32,
    pub rvclr : i32,
    pub rvcyc : i32,
    pub rvsnd : i32,
    pub rvgua : i32,
    pub orrug : i32,
    pub orcand : i32,
    pub ormtch : i32,
    pub orlamp : i32,
    pub mdir : i32,
    pub mloc : i32,
    pub poleuf : i32,
    pub quesno : i32,
    pub nqatt : i32,
    pub corrct : i32,
    pub lcell : i32,
    pub pnumb : i32,
    pub acell : i32,
    pub dcell : i32,
    pub cphere : i32,
}

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct8 {
    pub moves : i32,
    pub deaths : i32,
    pub rwscor : i32,
    pub mxscor : i32,
    pub mxload : i32,
    pub ltshft : i32,
    pub bloc : i32,
    pub mungrm : i32,
    pub hs : i32,
    pub egscor : i32,
    pub egmxsc : i32,
}

impl Clone for Struct8 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn score_(mut flg : i32) {
    static mut rank
        : [i32; 10]
        = [   20i32,
              19i32,
              18i32,
              16i32,
              12i32,
              8i32,
              4i32,
              2i32,
              1i32,
              0i32
          ];
    static mut erank : [i32; 5] = [ 20i32, 15i32, 10i32, 5i32, 0i32 ];
    let mut i__1 : i32;
    let mut i : i32;
    let mut i_2 : i32;
    i_2 = advs_.ascore[(play_.winner - 1i32) as (usize)];
    if findex_.endgmf != 0 {
        more_output(0i32 as (*mut ::std::os::raw::c_void) as (*const u8));
        printf((*b"Your score in the endgame \0").as_ptr());
        if flg != 0 {
            printf((*b"would be\0").as_ptr());
        } else {
            printf((*b"is\0").as_ptr());
        }
        printf(
            (*b" %d [total of %d points], in %d moves.\n\0").as_ptr(),
            state_.egscor,
            state_.egmxsc,
            state_.moves
        );
        i = 1i32;
        'loop15: loop {
            if !(i <= 5i32) {
                break;
            }
            if state_.egscor * 20i32 / state_.egmxsc >= erank[
                                                            (i - 1i32) as (usize)
                                                        ] {
                break;
            }
            i = i + 1;
        }
        i__1 = i + 786i32;
        rspeak_(i__1);
    } else {
        more_output(0i32 as (*mut ::std::os::raw::c_void) as (*const u8));
        printf((*b"Your score \0").as_ptr());
        if flg != 0 {
            printf((*b"would be\0").as_ptr());
        } else {
            printf((*b"is\0").as_ptr());
        }
        printf(
            (*b" %d [total of %d points], in %d move\0").as_ptr(),
            i_2,
            state_.mxscor,
            state_.moves
        );
        if state_.moves != 1i32 {
            printf((*b"s\0").as_ptr());
        }
        printf((*b".\n\0").as_ptr());
        i = 1i32;
        'loop7: loop {
            if !(i <= 10i32) {
                break;
            }
            if i_2 * 20i32 / state_.mxscor >= rank[(i - 1i32) as (usize)] {
                break;
            }
            i = i + 1;
        }
        i__1 = i + 484i32;
        rspeak_(i__1);
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct9 {
    pub clnt : i32,
    pub ctick : [i32; 25],
    pub cactio : [i32; 25],
    pub cflag : [i32; 25],
}

impl Clone for Struct9 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct10 {
    pub cevcur : i32,
    pub cevmnt : i32,
    pub cevlnt : i32,
    pub cevmat : i32,
    pub cevcnd : i32,
    pub cevbal : i32,
    pub cevbrn : i32,
    pub cevfus : i32,
    pub cevled : i32,
    pub cevsaf : i32,
    pub cevvlg : i32,
    pub cevgno : i32,
    pub cevbuc : i32,
    pub cevsph : i32,
    pub cevegh : i32,
    pub cevfor : i32,
    pub cevscl : i32,
    pub cevzgi : i32,
    pub cevzgo : i32,
    pub cevste : i32,
    pub cevmrs : i32,
    pub cevpin : i32,
    pub cevinq : i32,
    pub cevfol : i32,
}

impl Clone for Struct10 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn scrupd_(mut n : i32) {
    if findex_.endgmf != 0 {
        state_.egscor = state_.egscor + n;
    } else {
        let _rhs = n;
        let _lhs = &mut advs_.ascore[(play_.winner - 1i32) as (usize)];
        *_lhs = *_lhs + _rhs;
        state_.rwscor = state_.rwscor + n;
        (if advs_.ascore[
                (play_.winner - 1i32) as (usize)
            ] < state_.mxscor - state_.deaths * 10i32 {
         } else {
             cevent_.cflag[(cindex_.cevegh - 1i32) as (usize)] = 1i32;
             cevent_.ctick[(cindex_.cevegh - 1i32) as (usize)] = 15i32;
         })
    }
}
