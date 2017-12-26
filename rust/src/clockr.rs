extern {
    static mut advs_ : Struct1;
    static aindex_ : Struct2;
    fn bug_(arg1 : i32, arg2 : i32);
    static mut cevent_ : Struct3;
    static cindex_ : Struct4;
    static mut curxt_ : Struct14;
    static mut findex_ : Struct7;
    fn findxt_(arg1 : i32, arg2 : i32) -> i32;
    static mut hack_ : Struct12;
    fn jigsup_(arg1 : i32);
    fn lit_(arg1 : i32) -> i32;
    fn moveto_(arg1 : i32, arg2 : i32) -> i32;
    fn mrhere_(arg1 : i32) -> i32;
    fn newsta_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    );
    static mut objcts_ : Struct10;
    static oindex_ : Struct9;
    static mut play_ : Struct5;
    fn prob_(arg1 : i32, arg2 : i32) -> i32;
    fn qhere_(arg1 : i32, arg2 : i32) -> i32;
    static rindex_ : Struct6;
    fn rmdesc_(arg1 : i32) -> i32;
    static mut rooms_ : Struct8;
    fn rspeak_(arg1 : i32);
    fn rspsub_(arg1 : i32, arg2 : i32);
    fn scrupd_(arg1 : i32);
    static mut state_ : Struct11;
    static xsrch_ : Struct13;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
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
    pub clnt : i32,
    pub ctick : [i32; 25],
    pub cactio : [i32; 25],
    pub cflag : [i32; 25],
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
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

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub winner : i32,
    pub here : i32,
    pub telflg : i32,
}

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
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

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
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
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct8 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct9 {
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

impl Clone for Struct9 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct10 {
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

impl Clone for Struct10 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct11 {
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

impl Clone for Struct11 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct12 {
    pub thfpos : i32,
    pub thfflg : i32,
    pub thfact : i32,
    pub swdact : i32,
    pub swdsta : i32,
}

impl Clone for Struct12 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct13 {
    pub xmin : i32,
    pub xmax : i32,
    pub xdown : i32,
    pub xup : i32,
    pub xnorth : i32,
    pub xsouth : i32,
    pub xenter : i32,
    pub xexit : i32,
    pub xeast : i32,
    pub xwest : i32,
}

impl Clone for Struct13 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct14 {
    pub xtype : i32,
    pub xroom1 : i32,
    pub xstrng : i32,
    pub xactio : i32,
    pub xobj : i32,
}

impl Clone for Struct14 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn cevapp_(mut ri : i32) {
    let mut _currentBlock;
    static mut cndtck
        : [i32; 10]
        = [   50i32,
              20i32,
              10i32,
              5i32,
              0i32,
              156i32,
              156i32,
              156i32,
              157i32,
              0i32
          ];
    static mut lmptck
        : [i32; 12]
        = [   50i32,
              30i32,
              20i32,
              10i32,
              4i32,
              0i32,
              154i32,
              154i32,
              154i32,
              154i32,
              155i32,
              0i32
          ];
    let mut i__1 : i32;
    let mut i__2 : i32;
    let mut f : i32;
    let mut i : i32;
    let mut j : i32;
    let mut bc : i32;
    let mut br : i32;
    if ri == 0i32 {
    } else if ri == 24i32 {
        (if advs_.aroom[(aindex_.amastr - 1i32) as (usize)] == play_.here {
         } else if play_.here != rindex_.cell && (play_.here != rindex_.pcell) {
             findex_.follwf = 1i32;
             i = 812i32;
             i__1 = xsrch_.xmax;
             i__2 = xsrch_.xmin;
             j = xsrch_.xmin;
             'loop176: loop {
                 if if i__2 < 0i32 {
                        (j >= i__1) as (i32)
                    } else {
                        (j <= i__1) as (i32)
                    } == 0 {
                     break;
                 }
                 if findxt_(
                        j,
                        advs_.aroom[(aindex_.amastr - 1i32) as (usize)]
                    ) != 0 && (curxt_.xroom1 == play_.here) {
                     i = 813i32;
                 }
                 j = j + i__2;
             }
             rspeak_(i);
             newsta_(oindex_.master,0i32,play_.here,0i32,0i32);
             advs_.aroom[(aindex_.amastr - 1i32) as (usize)] = play_.here;
         } else {
             if findex_.follwf != 0 {
                 rspeak_(811i32);
             }
             findex_.follwf = 0i32;
         })
    } else if ri == 23i32 {
        (if advs_.aroom[
                (aindex_.player - 1i32) as (usize)
            ] != rindex_.fdoor {
         } else {
             rspeak_(769i32);
             i__1 = findex_.quesno + 770i32;
             rspeak_(i__1);
             cevent_.ctick[(cindex_.cevinq - 1i32) as (usize)] = 2i32;
         })
    } else if ri == 22i32 {
        if findex_.wdopnf != 0 {
            rspeak_(730i32);
        }
        findex_.wdopnf = 0i32;
    } else if ri == 21i32 {
        findex_.mrpshf = 0i32;
        findex_.mropnf = 0i32;
        if play_.here == rindex_.mrant {
            rspeak_(728i32);
        }
        if play_.here == rindex_.inmir || mrhere_(play_.here) == 1i32 {
            rspeak_(729i32);
        }
    } else if ri == 20i32 {
        if findex_.spellf == 0 {
            if play_.here != rindex_.crypt {
                return;
            } else if lit_(play_.here) == 0 {
                rspeak_(727i32);
            } else {
                cevent_.ctick[(cindex_.cevste - 1i32) as (usize)] = 3i32;
                return;
            }
        }
        i__1 = objcts_.olnt;
        i = 1i32;
        'loop155: loop {
            if !(i <= i__1) {
                break;
            }
            newsta_(
                i,
                0i32,
                objcts_.oroom[(i - 1i32) as (usize)],
                objcts_.ocan[(i - 1i32) as (usize)],
                0i32
            );
            i = i + 1;
        }
        newsta_(oindex_.lamp,0i32,0i32,0i32,aindex_.player);
        newsta_(oindex_.sword,0i32,0i32,0i32,aindex_.player);
        objcts_.oflag1[(oindex_.lamp - 1i32) as (usize)] = (objcts_.oflag1[
                                                                (oindex_.lamp - 1i32) as (usize)
                                                            ] | 64i32) & !1i32;
        let _rhs = 4i32;
        let _lhs = &mut objcts_.oflag2[(oindex_.lamp - 1i32) as (usize)];
        *_lhs = *_lhs | _rhs;
        cevent_.cflag[(cindex_.cevlnt - 1i32) as (usize)] = 0i32;
        cevent_.ctick[(cindex_.cevlnt - 1i32) as (usize)] = 350i32;
        findex_.orlamp = 0i32;
        let _rhs = 4i32;
        let _lhs = &mut objcts_.oflag2[(oindex_.sword - 1i32) as (usize)];
        *_lhs = *_lhs | _rhs;
        hack_.swdact = 1i32;
        hack_.swdsta = 0i32;
        hack_.thfact = 0i32;
        findex_.endgmf = 1i32;
        cevent_.cflag[(cindex_.cevmat - 1i32) as (usize)] = 0i32;
        cevent_.cflag[(cindex_.cevcnd - 1i32) as (usize)] = 0i32;
        scrupd_(rooms_.rval[(rindex_.crypt - 1i32) as (usize)]);
        rooms_.rval[(rindex_.crypt - 1i32) as (usize)] = 0i32;
        f = moveto_(rindex_.tstrs,play_.winner);
        f = rmdesc_(3i32);
    } else if ri == 19i32 {
        newsta_(oindex_.zgnom,0i32,0i32,0i32,0i32);
        if play_.here == rindex_.bktwi {
            rspeak_(638i32);
        }
    } else if ri == 18i32 {
        cevent_.cflag[(cindex_.cevzgo - 1i32) as (usize)] = 1i32;
        newsta_(oindex_.zgnom,0i32,rindex_.bktwi,0i32,0i32);
        if play_.here == rindex_.bktwi {
            rspeak_(637i32);
        }
    } else if ri == 17i32 {
        if play_.here == rindex_.bktwi {
            cevent_.cflag[(cindex_.cevzgi - 1i32) as (usize)] = 1i32;
        }
        if play_.here == rindex_.bkvau {
            jigsup_(636i32);
        }
    } else if ri == 16i32 {
        cevent_.cflag[
            (cindex_.cevfor - 1i32) as (usize)
        ] = (play_.here == rindex_.mtree || play_.here >= rindex_.fore1 && (play_.here < rindex_.clear)) as (i32);
        if cevent_.cflag[
               (cindex_.cevfor - 1i32) as (usize)
           ] != 0 && (prob_(10i32,10i32) != 0) {
            rspeak_(635i32);
        }
    } else if ri == 15i32 {
        findex_.endgmf = 1i32;
        rspeak_(119i32);
    } else if ri == 14i32 {
        let _rhs = 256i32;
        let _lhs = &mut rooms_.rflag[(rindex_.cager - 1i32) as (usize)];
        *_lhs = *_lhs | _rhs;
        *(&mut rooms_ as (*mut Struct8) as (*mut i32)).offset(
             601isize
         ).offset(
             (rindex_.cager - 1i32) as (isize)
         ) = 147i32;
        jigsup_(148i32);
    } else if ri == 13i32 {
        if objcts_.ocan[
               (oindex_.water - 1i32) as (usize)
           ] == oindex_.bucke {
            newsta_(oindex_.water,0i32,0i32,0i32,0i32);
        }
    } else if ri == 12i32 {
        newsta_(oindex_.gnome,149i32,0i32,0i32,0i32);
    } else if ri == 11i32 {
        (if play_.here == rindex_.ledg2 || play_.here == rindex_.ledg3 || play_.here == rindex_.ledg4 || play_.here == rindex_.vlbot {
             newsta_(oindex_.gnome,118i32,play_.here,0i32,0i32);
         } else {
             cevent_.ctick[(cindex_.cevvlg - 1i32) as (usize)] = 1i32;
         })
    } else if ri == 10i32 {
        let _rhs = 256i32;
        let _lhs = &mut rooms_.rflag[(state_.mungrm - 1i32) as (usize)];
        *_lhs = *_lhs | _rhs;
        *(&mut rooms_ as (*mut Struct8) as (*mut i32)).offset(
             601isize
         ).offset(
             (state_.mungrm - 1i32) as (isize)
         ) = 114i32;
        (if play_.here == state_.mungrm {
             i = 116i32;
             if rooms_.rflag[(play_.here - 1i32) as (usize)] & 64i32 != 0i32 {
                 i = 117i32;
             }
             jigsup_(i);
         } else {
             rspeak_(115i32);
             if state_.mungrm == rindex_.msafe {
                 cevent_.ctick[(cindex_.cevled - 1i32) as (usize)] = 8i32;
             }
         })
    } else if ri == 9i32 {
        let _rhs = 256i32;
        let _lhs = &mut rooms_.rflag[(rindex_.ledg4 - 1i32) as (usize)];
        *_lhs = *_lhs | _rhs;
        *(&mut rooms_ as (*mut Struct8) as (*mut i32)).offset(
             601isize
         ).offset(
             (rindex_.ledg4 - 1i32) as (isize)
         ) = 109i32;
        (if play_.here == rindex_.ledg4 {
             (if advs_.avehic[(play_.winner - 1i32) as (usize)] != 0i32 {
                  (if findex_.btief != 0i32 {
                       state_.bloc = rindex_.vlbot;
                       newsta_(oindex_.ballo,0i32,0i32,0i32,0i32);
                       newsta_(oindex_.dball,0i32,state_.bloc,0i32,0i32);
                       findex_.btief = 0i32;
                       findex_.binff = 0i32;
                       cevent_.cflag[(cindex_.cevbal - 1i32) as (usize)] = 0i32;
                       cevent_.cflag[(cindex_.cevbrn - 1i32) as (usize)] = 0i32;
                       jigsup_(113i32);
                   } else {
                       rspeak_(112i32);
                   })
              } else {
                  jigsup_(111i32);
              })
         } else {
             rspeak_(110i32);
         })
    } else if ri == 8i32 {
        (if objcts_.ocan[
                (oindex_.fuse - 1i32) as (usize)
            ] != oindex_.brick {
             if qhere_(oindex_.fuse,play_.here) != 0 || objcts_.oadv[
                                                            (oindex_.fuse - 1i32) as (usize)
                                                        ] == play_.winner {
                 rspeak_(152i32);
             }
             newsta_(oindex_.fuse,0i32,0i32,0i32,0i32);
         } else {
             br = objcts_.oroom[(oindex_.brick - 1i32) as (usize)];
             bc = objcts_.ocan[(oindex_.brick - 1i32) as (usize)];
             if br == 0i32 && (bc != 0i32) {
                 br = objcts_.oroom[(bc - 1i32) as (usize)];
             }
             newsta_(oindex_.fuse,0i32,0i32,0i32,0i32);
             newsta_(oindex_.brick,0i32,0i32,0i32,0i32);
             (if br != 0i32 && (br != play_.here) {
                  rspeak_(151i32);
                  state_.mungrm = br;
                  cevent_.ctick[(cindex_.cevsaf - 1i32) as (usize)] = 5i32;
                  (if br != rindex_.msafe {
                       i__1 = objcts_.olnt;
                       i = 1i32;
                       'loop96: loop {
                           if !(i <= i__1) {
                               break;
                           }
                           if qhere_(i,br) != 0 && (objcts_.oflag1[
                                                        (i - 1i32) as (usize)
                                                    ] & 8192i32 != 0i32) {
                               newsta_(i,0i32,0i32,0i32,0i32);
                           }
                           i = i + 1;
                       }
                       (if br != rindex_.lroom {
                        } else {
                            i__1 = objcts_.olnt;
                            i = 1i32;
                            'loop99: loop {
                                if !(i <= i__1) {
                                    break;
                                }
                                if objcts_.ocan[(i - 1i32) as (usize)] == oindex_.tcase {
                                    newsta_(i,0i32,0i32,0i32,0i32);
                                }
                                i = i + 1;
                            }
                        })
                   } else if bc != oindex_.sslot {
                   } else {
                       newsta_(oindex_.sslot,0i32,0i32,0i32,0i32);
                       let _rhs = 8i32;
                       let _lhs = &mut objcts_.oflag2[(oindex_.safe - 1i32) as (usize)];
                       *_lhs = *_lhs | _rhs;
                       findex_.safef = 1i32;
                   })
              } else {
                  let _rhs = 256i32;
                  let _lhs = &mut rooms_.rflag[(play_.here - 1i32) as (usize)];
                  *_lhs = *_lhs | _rhs;
                  *(&mut rooms_ as (*mut Struct8) as (*mut i32)).offset(
                       601isize
                   ).offset(
                       (play_.here - 1i32) as (isize)
                   ) = 114i32;
                  jigsup_(150i32);
              })
         })
    } else if ri == 7i32 {
        i__1 = objcts_.olnt;
        i = 1i32;
        'loop79: loop {
            if !(i <= i__1) {
                _currentBlock = 80;
                break;
            }
            if oindex_.recep == objcts_.ocan[
                                    (i - 1i32) as (usize)
                                ] && (objcts_.oflag1[(i - 1i32) as (usize)] & 8i32 != 0i32) {
                _currentBlock = 83;
                break;
            }
            i = i + 1;
        }
        if _currentBlock == 80 {
            bug_(4i32,0i32);
        }
        newsta_(i,0i32,0i32,0i32,0i32);
        findex_.binff = 0i32;
        if play_.here == state_.bloc {
            rspsub_(292i32,objcts_.odesc2[(i - 1i32) as (usize)]);
        }
    } else if ri == 6i32 {
        cevent_.ctick[(cindex_.cevbal - 1i32) as (usize)] = 3i32;
        f = (advs_.avehic[
                 (play_.winner - 1i32) as (usize)
             ] == oindex_.ballo) as (i32);
        (if state_.bloc == rindex_.vlbot {
             (if findex_.binff == 0i32 || !(objcts_.oflag2[
                                                (oindex_.recep - 1i32) as (usize)
                                            ] & 8i32 != 0i32) {
              } else {
                  state_.bloc = rindex_.vair1;
                  newsta_(oindex_.ballo,0i32,state_.bloc,0i32,0i32);
                  (if f != 0 {
                       f = moveto_(state_.bloc,play_.winner);
                       rspeak_(542i32);
                       f = rmdesc_(0i32);
                   } else if play_.here == rindex_.ledg2 || play_.here == rindex_.ledg3 || play_.here == rindex_.ledg4 || play_.here == rindex_.vlbot {
                       rspeak_(541i32);
                   })
              })
         } else if state_.bloc == rindex_.ledg2 || state_.bloc == rindex_.ledg3 || state_.bloc == rindex_.ledg4 || state_.bloc == rindex_.vlbot {
             state_.bloc = state_.bloc + (rindex_.vair2 - rindex_.ledg2);
             newsta_(oindex_.ballo,0i32,state_.bloc,0i32,0i32);
             (if f != 0 {
                  f = moveto_(state_.bloc,play_.winner);
                  rspeak_(540i32);
                  f = rmdesc_(0i32);
              } else {
                  if play_.here == rindex_.ledg2 || play_.here == rindex_.ledg3 || play_.here == rindex_.ledg4 || play_.here == rindex_.vlbot {
                      rspeak_(539i32);
                  }
                  cevent_.ctick[(cindex_.cevvlg - 1i32) as (usize)] = 10i32;
              })
         } else if objcts_.oflag2[
                       (oindex_.recep - 1i32) as (usize)
                   ] & 8i32 != 0i32 && (findex_.binff != 0i32) {
             (if state_.bloc != rindex_.vair4 {
                  state_.bloc = state_.bloc + 1;
                  newsta_(oindex_.ballo,0i32,state_.bloc,0i32,0i32);
                  (if f != 0 {
                       f = moveto_(state_.bloc,play_.winner);
                       rspeak_(538i32);
                       f = rmdesc_(0i32);
                   } else if play_.here == rindex_.ledg2 || play_.here == rindex_.ledg3 || play_.here == rindex_.ledg4 || play_.here == rindex_.vlbot {
                       rspeak_(537i32);
                   })
              } else {
                  cevent_.ctick[(cindex_.cevbrn - 1i32) as (usize)] = 0i32;
                  cevent_.ctick[(cindex_.cevbal - 1i32) as (usize)] = 0i32;
                  findex_.binff = 0i32;
                  findex_.btief = 0i32;
                  state_.bloc = rindex_.vlbot;
                  newsta_(oindex_.ballo,0i32,0i32,0i32,0i32);
                  newsta_(oindex_.dball,0i32,state_.bloc,0i32,0i32);
                  (if f != 0 {
                       jigsup_(536i32);
                   } else if play_.here == rindex_.ledg2 || play_.here == rindex_.ledg3 || play_.here == rindex_.ledg4 || play_.here == rindex_.vlbot {
                       rspeak_(535i32);
                   })
              })
         } else if state_.bloc != rindex_.vair1 {
             state_.bloc = state_.bloc - 1;
             newsta_(oindex_.ballo,0i32,state_.bloc,0i32,0i32);
             (if f != 0 {
                  f = moveto_(state_.bloc,play_.winner);
                  rspeak_(534i32);
                  f = rmdesc_(0i32);
              } else if play_.here == rindex_.ledg2 || play_.here == rindex_.ledg3 || play_.here == rindex_.ledg4 || play_.here == rindex_.vlbot {
                  rspeak_(533i32);
              })
         } else {
             state_.bloc = rindex_.vlbot;
             newsta_(oindex_.ballo,0i32,state_.bloc,0i32,0i32);
             (if f != 0 {
                  f = moveto_(state_.bloc,play_.winner);
                  (if findex_.binff == 0i32 {
                       newsta_(oindex_.ballo,532i32,0i32,0i32,0i32);
                       newsta_(oindex_.dball,0i32,state_.bloc,0i32,0i32);
                       advs_.avehic[(play_.winner - 1i32) as (usize)] = 0i32;
                       cevent_.cflag[(cindex_.cevbal - 1i32) as (usize)] = 0i32;
                       cevent_.cflag[(cindex_.cevbrn - 1i32) as (usize)] = 0i32;
                       findex_.binff = 0i32;
                       findex_.btief = 0i32;
                   } else {
                       rspeak_(531i32);
                       f = rmdesc_(0i32);
                   })
              } else if play_.here == rindex_.ledg2 || play_.here == rindex_.ledg3 || play_.here == rindex_.ledg4 || play_.here == rindex_.vlbot {
                  rspeak_(530i32);
              })
         })
    } else if ri == 5i32 {
        litint_(
            oindex_.candl,
            &mut findex_.orcand as (*mut i32),
            cindex_.cevcnd,
            cndtck.as_ptr(),
            10i32
        );
    } else if ri == 4i32 {
        rspeak_(153i32);
        let _rhs = !1i32;
        let _lhs = &mut objcts_.oflag1[(oindex_.match_ - 1i32) as (usize)];
        *_lhs = *_lhs & _rhs;
    } else if ri == 3i32 {
        litint_(
            oindex_.lamp,
            &mut findex_.orlamp as (*mut i32),
            cindex_.cevlnt,
            lmptck.as_ptr(),
            12i32
        );
    } else if ri == 2i32 {
        if play_.here == rindex_.maint {
            i__1 = findex_.rvmnt / 2i32 + 71i32;
            rspeak_(i__1);
        }
        findex_.rvmnt = findex_.rvmnt + 1;
        (if findex_.rvmnt <= 16i32 {
         } else {
             cevent_.ctick[(cindex_.cevmnt - 1i32) as (usize)] = 0i32;
             let _rhs = 256i32;
             let _lhs = &mut rooms_.rflag[(rindex_.maint - 1i32) as (usize)];
             *_lhs = *_lhs | _rhs;
             *(&mut rooms_ as (*mut Struct8) as (*mut i32)).offset(
                  601isize
              ).offset(
                  (rindex_.maint - 1i32) as (isize)
              ) = 80i32;
             if play_.here == rindex_.maint {
                 jigsup_(81i32);
             }
         })
    } else {
        if !(ri == 1i32) {
            bug_(3i32,ri);
        }
        i__1 = 0i32;
        i__2 = advs_.astren[(aindex_.player - 1i32) as (usize)] + 1i32;
        advs_.astren[
            (aindex_.player - 1i32) as (usize)
        ] = if i__1 <= i__2 { i__1 } else { i__2 };
        (if advs_.astren[(aindex_.player - 1i32) as (usize)] >= 0i32 {
         } else {
             cevent_.ctick[(cindex_.cevcur - 1i32) as (usize)] = 30i32;
         })
    }
}

unsafe extern fn litint_(
    mut obj : i32,
    mut ctr : *mut i32,
    mut cev : i32,
    mut ticks : *const i32,
    mut tickln : i32
) {
    ticks = ticks.offset(-1isize);
    *ctr = *ctr + 1;
    cevent_.ctick[(cev - 1i32) as (usize)] = *ticks.offset(
                                                  *ctr as (isize)
                                              );
    if cevent_.ctick[(cev - 1i32) as (usize)] != 0i32 {
        if objcts_.oroom[
               (obj - 1i32) as (usize)
           ] == play_.here || objcts_.oadv[
                                  (obj - 1i32) as (usize)
                              ] == play_.winner {
            rspeak_(*ticks.offset((*ctr + tickln / 2i32) as (isize)));
        }
    } else {
        let _rhs = !(64i32 + 8i32 + 1i32);
        let _lhs = &mut objcts_.oflag1[(obj - 1i32) as (usize)];
        *_lhs = *_lhs & _rhs;
        if objcts_.oroom[
               (obj - 1i32) as (usize)
           ] == play_.here || objcts_.oadv[
                                  (obj - 1i32) as (usize)
                              ] == play_.winner {
            rspsub_(293i32,objcts_.odesc2[(obj - 1i32) as (usize)]);
        }
    }
}
