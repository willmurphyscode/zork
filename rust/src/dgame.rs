extern {
    fn aappli_(arg1 : i32) -> i32;
    static mut advs_ : Struct12;
    static aindex_ : Struct2;
    fn clockd_() -> i32;
    fn fightd_();
    static mut findex_ : Struct8;
    fn findxt_(arg1 : i32, arg2 : i32) -> i32;
    static mut hack_ : Struct13;
    static mut input_ : Struct4;
    fn lit_(arg1 : i32) -> i32;
    fn more_output(arg1 : *const u8);
    fn oactor_(arg1 : i32) -> i32;
    fn oappli_(arg1 : i32, arg2 : i32) -> i32;
    static mut objcts_ : Struct11;
    static oindex_ : Struct7;
    fn parse_(arg1 : *mut u8, arg2 : i32) -> i32;
    static mut play_ : Struct1;
    static mut prsvec_ : Struct3;
    fn rappli_(arg1 : i32) -> i32;
    fn rdline_(arg1 : *mut u8, arg2 : i32);
    static rindex_ : Struct9;
    fn rmdesc_(arg1 : i32) -> i32;
    static mut rooms_ : Struct10;
    fn rspeak_(arg1 : i32);
    static mut state_ : Struct5;
    fn strcmp(arg1 : *const u8, arg2 : *const u8) -> i32;
    fn swordd_();
    fn thiefd_();
    fn valuac_(arg1 : i32);
    fn vappli_(arg1 : i32) -> i32;
    static vindex_ : Struct6;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub winner : i32,
    pub here : i32,
    pub telflg : i32,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub player : i32,
    pub arobot : i32,
    pub amastr : i32,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub prsa : i32,
    pub prsi : i32,
    pub prso : i32,
    pub prswon : i32,
    pub prscon : i32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub inlnt : i32,
    pub inbuf : [u8; 78],
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
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

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
    pub cintw : i32,
    pub deadxw : i32,
    pub frstqw : i32,
    pub inxw : i32,
    pub outxw : i32,
    pub walkiw : i32,
    pub fightw : i32,
    pub foow : i32,
    pub meltw : i32,
    pub readw : i32,
    pub inflaw : i32,
    pub deflaw : i32,
    pub alarmw : i32,
    pub exorcw : i32,
    pub plugw : i32,
    pub kickw : i32,
    pub wavew : i32,
    pub raisew : i32,
    pub lowerw : i32,
    pub rubw : i32,
    pub pushw : i32,
    pub untiew : i32,
    pub tiew : i32,
    pub tieupw : i32,
    pub turnw : i32,
    pub breatw : i32,
    pub knockw : i32,
    pub lookw : i32,
    pub examiw : i32,
    pub shakew : i32,
    pub movew : i32,
    pub trnonw : i32,
    pub trnofw : i32,
    pub openw : i32,
    pub closew : i32,
    pub findw : i32,
    pub waitw : i32,
    pub spinw : i32,
    pub boardw : i32,
    pub unboaw : i32,
    pub takew : i32,
    pub invenw : i32,
    pub fillw : i32,
    pub eatw : i32,
    pub drinkw : i32,
    pub burnw : i32,
    pub mungw : i32,
    pub killw : i32,
    pub attacw : i32,
    pub swingw : i32,
    pub walkw : i32,
    pub tellw : i32,
    pub putw : i32,
    pub dropw : i32,
    pub givew : i32,
    pub pourw : i32,
    pub throww : i32,
    pub digw : i32,
    pub leapw : i32,
    pub stayw : i32,
    pub follow : i32,
    pub hellow : i32,
    pub lookiw : i32,
    pub lookuw : i32,
    pub pumpw : i32,
    pub windw : i32,
    pub clmbw : i32,
    pub clmbuw : i32,
    pub clmbdw : i32,
    pub trntow : i32,
}

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
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

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct8 {
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

impl Clone for Struct8 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct9 {
    pub whous : i32,
    pub lroom : i32,
    pub cella : i32,
    pub mtrol : i32,
    pub maze1 : i32,
    pub mgrat : i32,
    pub maz15 : i32,
    pub fore1 : i32,
    pub fore3 : i32,
    pub clear : i32,
    pub reser : i32,
    pub strea : i32,
    pub egypt : i32,
    pub echor : i32,
    pub tshaf : i32,
    pub bshaf : i32,
    pub mmach : i32,
    pub dome : i32,
    pub mtorc : i32,
    pub carou : i32,
    pub riddl : i32,
    pub lld2 : i32,
    pub temp1 : i32,
    pub temp2 : i32,
    pub maint : i32,
    pub blroo : i32,
    pub treas : i32,
    pub rivr1 : i32,
    pub rivr2 : i32,
    pub rivr3 : i32,
    pub mcycl : i32,
    pub rivr4 : i32,
    pub rivr5 : i32,
    pub fchmp : i32,
    pub falls : i32,
    pub mbarr : i32,
    pub mrain : i32,
    pub pog : i32,
    pub vlbot : i32,
    pub vair1 : i32,
    pub vair2 : i32,
    pub vair3 : i32,
    pub vair4 : i32,
    pub ledg2 : i32,
    pub ledg3 : i32,
    pub ledg4 : i32,
    pub msafe : i32,
    pub cager : i32,
    pub caged : i32,
    pub twell : i32,
    pub bwell : i32,
    pub alice : i32,
    pub alism : i32,
    pub alitr : i32,
    pub mtree : i32,
    pub bkent : i32,
    pub bkvw : i32,
    pub bktwi : i32,
    pub bkvau : i32,
    pub bkbox : i32,
    pub crypt : i32,
    pub tstrs : i32,
    pub mrant : i32,
    pub mreye : i32,
    pub mra : i32,
    pub mrb : i32,
    pub mrc : i32,
    pub mrg : i32,
    pub mrd : i32,
    pub fdoor : i32,
    pub mrae : i32,
    pub mrce : i32,
    pub mrcw : i32,
    pub mrge : i32,
    pub mrgw : i32,
    pub mrdw : i32,
    pub inmir : i32,
    pub scorr : i32,
    pub ncorr : i32,
    pub parap : i32,
    pub cell : i32,
    pub pcell : i32,
    pub ncell : i32,
    pub cpant : i32,
    pub cpout : i32,
    pub cpuzz : i32,
}

impl Clone for Struct9 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct10 {
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct10 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct11 {
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

impl Clone for Struct11 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct12 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct12 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn game_() {
    let mut _currentBlock;
    let mut f : i32;
    let mut i : i32;
    rspeak_(1i32);
    f = rmdesc_(3i32);
    'loop1: loop {
        play_.winner = aindex_.player;
        play_.telflg = 0i32;
        if prsvec_.prscon <= 1i32 {
            rdline_(input_.inbuf.as_mut_ptr(),1i32);
        }
        state_.moves = state_.moves + 1;
        prsvec_.prswon = parse_(input_.inbuf.as_mut_ptr(),1i32);
        if !(prsvec_.prswon == 0) {
            if xvehic_(1i32) == 0 {
                if prsvec_.prsa == vindex_.tellw {
                    if objcts_.oflag2[
                           (prsvec_.prso - 1i32) as (usize)
                       ] & 1024i32 != 0i32 {
                        play_.winner = oactor_(prsvec_.prso);
                        play_.here = advs_.aroom[(play_.winner - 1i32) as (usize)];
                        if prsvec_.prscon <= 1i32 {
                            _currentBlock = 17;
                        } else if parse_(input_.inbuf.as_mut_ptr(),1i32) != 0 {
                            if aappli_(advs_.aactio[(play_.winner - 1i32) as (usize)]) != 0 {
                                _currentBlock = 16;
                            } else if xvehic_(1i32) != 0 {
                                _currentBlock = 16;
                            } else if prsvec_.prso == oindex_.valua || prsvec_.prso == oindex_.every {
                                valuac_(oindex_.valua);
                                _currentBlock = 21;
                            } else if vappli_(prsvec_.prsa) == 0 {
                                _currentBlock = 16;
                            } else {
                                f = rappli_(rooms_.ractio[(play_.here - 1i32) as (usize)]);
                                _currentBlock = 16;
                            }
                            if _currentBlock == 21 {
                            } else {
                                xendmv_(play_.telflg);
                                _currentBlock = 20;
                            }
                        } else {
                            _currentBlock = 17;
                        }
                        if _currentBlock == 21 {
                        } else {
                            if _currentBlock == 17 {
                                i = 341i32;
                                if play_.telflg != 0 {
                                    i = 604i32;
                                }
                                rspeak_(i);
                            }
                            play_.winner = aindex_.player;
                            play_.here = advs_.aroom[(play_.winner - 1i32) as (usize)];
                        }
                    } else {
                        rspeak_(602i32);
                    }
                    _currentBlock = 21;
                } else {
                    _currentBlock = 27;
                }
                'loop21: loop {
                    if _currentBlock == 21 {
                        if !(findex_.echof == 0 && (play_.here == rindex_.echor)) {
                            _currentBlock = 22;
                            break;
                        }
                        'loop23: loop {
                            rdline_(input_.inbuf.as_mut_ptr(),0i32);
                            state_.moves = state_.moves + 1;
                            if !(strcmp(
                                     input_.inbuf.as_mut_ptr() as (*const u8),
                                     (*b"ECHO\0").as_ptr()
                                 ) != 0i32) {
                                _currentBlock = 24;
                                break 'loop21;
                            }
                            prsvec_.prswon = parse_(input_.inbuf.as_mut_ptr(),0i32);
                            if !(prsvec_.prswon == 0 || prsvec_.prsa != vindex_.walkw) {
                                if findxt_(prsvec_.prso,play_.here) != 0 {
                                    _currentBlock = 27;
                                    break;
                                }
                            }
                            more_output(input_.inbuf.as_mut_ptr() as (*const u8));
                            play_.telflg = 1i32;
                        }
                    } else if prsvec_.prso == oindex_.valua || prsvec_.prso == oindex_.every {
                        valuac_(oindex_.valua);
                        _currentBlock = 21;
                    } else if vappli_(prsvec_.prsa) == 0 {
                        _currentBlock = 31;
                        break;
                    } else {
                        _currentBlock = 21;
                    }
                }
                if _currentBlock == 31 {
                } else if _currentBlock == 22 {
                    f = rappli_(rooms_.ractio[(play_.here - 1i32) as (usize)]);
                } else {
                    rspeak_(571i32);
                    findex_.echof = 1i32;
                    let _rhs = !8192i32;
                    let _lhs = &mut objcts_.oflag2[(oindex_.bar - 1i32) as (usize)];
                    *_lhs = *_lhs & _rhs;
                    prsvec_.prswon = 1i32;
                    prsvec_.prscon = 1i32;
                }
            }
        }
        xendmv_(play_.telflg);
        if !(lit_(play_.here) == 0) {
            continue;
        }
        prsvec_.prscon = 1i32;
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct13 {
    pub thfpos : i32,
    pub thfflg : i32,
    pub thfact : i32,
    pub swdact : i32,
    pub swdsta : i32,
}

impl Clone for Struct13 {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn xendmv_(mut flag : i32) {
    let mut f : i32;
    if flag == 0 {
        rspeak_(341i32);
    }
    if hack_.thfact != 0 {
        thiefd_();
    }
    if prsvec_.prswon != 0 {
        fightd_();
    }
    if hack_.swdact != 0 {
        swordd_();
    }
    if prsvec_.prswon != 0 {
        f = clockd_();
    }
    if prsvec_.prswon != 0 {
        f = xvehic_(2i32);
    }
}

unsafe extern fn xvehic_(mut n : i32) -> i32 {
    let mut ret_val : i32;
    let mut av : i32;
    ret_val = 0i32;
    av = advs_.avehic[(play_.winner - 1i32) as (usize)];
    if av != 0i32 {
        ret_val = oappli_(objcts_.oactio[(av - 1i32) as (usize)],n);
    }
    ret_val
}
