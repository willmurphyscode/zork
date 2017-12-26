extern {
    static mut advs_ : Struct2;
    static mut findex_ : Struct5;
    fn newsta_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    );
    fn oactor_(arg1 : i32) -> i32;
    static mut objcts_ : Struct1;
    static oindex_ : Struct4;
    fn prob_(arg1 : i32, arg2 : i32) -> i32;
    static mut prsvec_ : Struct7;
    fn qhere_(arg1 : i32, arg2 : i32) -> i32;
    static mut state_ : Struct3;
    static mut vill_ : Struct6;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
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

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn robadv_(
    mut adv : i32, mut nr : i32, mut nc : i32, mut na : i32
) -> i32 {
    let mut ret_val : i32;
    let mut i__1 : i32;
    let mut i : i32;
    ret_val = 0i32;
    i__1 = objcts_.olnt;
    i = 1i32;
    'loop1: loop {
        if !(i <= i__1) {
            break;
        }
        if !(objcts_.oadv[(i - 1i32) as (usize)] != adv || objcts_.otval[
                                                               (i - 1i32) as (usize)
                                                           ] <= 0i32 || objcts_.oflag2[
                                                                            (i - 1i32) as (usize)
                                                                        ] & 8192i32 != 0i32) {
            newsta_(i,0i32,nr,nc,na);
            ret_val = ret_val + 1;
        }
        i = i + 1;
    }
    ret_val
}

#[no_mangle]
pub unsafe extern fn robrm_(
    mut rm : i32,
    mut pr : i32,
    mut nr : i32,
    mut nc : i32,
    mut na : i32
) -> i32 {
    let mut ret_val : i32;
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut i : i32;
    ret_val = 0i32;
    i__1 = objcts_.olnt;
    i = 1i32;
    'loop1: loop {
        if !(i <= i__1) {
            break;
        }
        if !(qhere_(i,rm) == 0) {
            if objcts_.otval[(i - 1i32) as (usize)] <= 0i32 || objcts_.oflag2[
                                                                   (i - 1i32) as (usize)
                                                               ] & 8192i32 != 0i32 || objcts_.oflag1[
                                                                                          (i - 1i32) as (usize)
                                                                                      ] & 32768i32 == 0i32 || prob_(
                                                                                                                  pr,
                                                                                                                  pr
                                                                                                              ) == 0 {
                if objcts_.oflag2[(i - 1i32) as (usize)] & 1024i32 != 0i32 {
                    i__2 = oactor_(i);
                    ret_val = ret_val + robadv_(i__2,nr,nc,na);
                }
            } else {
                newsta_(i,0i32,nr,nc,na);
                ret_val = ret_val + 1;
                let _rhs = 4i32;
                let _lhs = &mut objcts_.oflag2[(i - 1i32) as (usize)];
                *_lhs = *_lhs | _rhs;
            }
        }
        i = i + 1;
    }
    ret_val
}

#[no_mangle]
pub unsafe extern fn winnin_(mut vl : i32, mut hr : i32) -> i32 {
    let mut ret_val : i32;
    let mut ps : i32;
    let mut vs : i32;
    vs = objcts_.ocapac[(vl - 1i32) as (usize)];
    ps = vs - fights_(hr,1i32);
    ret_val = prob_(90i32,100i32);
    if ps > 3i32 {
        ret_val
    } else {
        ret_val = prob_(75i32,85i32);
        (if ps > 0i32 {
             ret_val
         } else {
             ret_val = prob_(50i32,30i32);
             (if ps == 0i32 {
                  ret_val
              } else {
                  ret_val = prob_(25i32,25i32);
                  (if vs > 1i32 {
                       ret_val
                   } else {
                       ret_val = prob_(10i32,0i32);
                       ret_val
                   })
              })
         })
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
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

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn fights_(mut h : i32, mut flg : i32) -> i32 {
    let smin : i32 = 2i32;
    let smax : i32 = 7i32;
    let mut ret_val : i32;
    ret_val = smin + ((smax - smin) * advs_.ascore[
                                          (h - 1i32) as (usize)
                                      ] + state_.mxscor / 2i32) / state_.mxscor;
    if flg != 0 {
        ret_val = ret_val + advs_.astren[(h - 1i32) as (usize)];
    }
    ret_val
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

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
    pub vlnt : i32,
    pub villns : [i32; 4],
    pub vprob : [i32; 4],
    pub vopps : [i32; 4],
    pub vbest : [i32; 4],
    pub vmelee : [i32; 4],
}

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
    pub prsa : i32,
    pub prsi : i32,
    pub prso : i32,
    pub prswon : i32,
    pub prscon : i32,
}

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn vilstr_(mut v : i32) -> i32 {
    let mut ret_val : i32;
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut i__3 : i32;
    let mut i : i32;
    ret_val = objcts_.ocapac[(v - 1i32) as (usize)];
    if ret_val <= 0i32 {
        ret_val
    } else {
        if !(v != oindex_.thief || findex_.thfenf == 0) {
            findex_.thfenf = 0i32;
            ret_val = if ret_val <= 2i32 { ret_val } else { 2i32 };
        }
        i__1 = vill_.vlnt;
        i = 1i32;
        'loop4: loop {
            if !(i <= i__1) {
                break;
            }
            if vill_.villns[
                   (i - 1i32) as (usize)
               ] == v && (prsvec_.prsi == vill_.vbest[(i - 1i32) as (usize)]) {
                i__2 = 1i32;
                i__3 = ret_val - 1i32;
                ret_val = if i__2 >= i__3 { i__2 } else { i__3 };
            }
            i = i + 1;
        }
        ret_val
    }
}
