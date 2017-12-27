extern {
    fn exit_();
    fn game_();
    fn init_() -> i32;
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct1 {
    pub prsa : i32,
    pub prsi : i32,
    pub prso : i32,
    pub prswon : i32,
    pub prscon : i32,
}

impl Clone for Struct1 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut prsvec_
    : Struct1
    = Struct1 {
          prsa: 0i32,
          prsi: 0i32,
          prso: 0i32,
          prswon: 0i32,
          prscon: 0i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub oflag : i32,
    pub oact : i32,
    pub oslot : i32,
    pub oprep : i32,
    pub oname : i32,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut orphs_
    : Struct2
    = Struct2 {
          oflag: 0i32,
          oact: 0i32,
          oslot: 0i32,
          oprep: 0i32,
          oname: 0i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub lastit : i32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut last_ : Struct3 = Struct3 { lastit: 0i32 };

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub winner : i32,
    pub here : i32,
    pub telflg : i32,
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut play_
    : Struct4
    = Struct4 { winner: 0i32, here: 0i32, telflg: 0i32 };

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut rooms_
    : Struct5
    = Struct5 {
          rlnt: 0i32,
          rdesc1: [0i32; 200],
          rdesc2: [0i32; 200],
          rexit: [0i32; 200],
          ractio: [0i32; 200],
          rval: [0i32; 200],
          rflag: [0i32; 200]
      };

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

#[no_mangle]
pub static rindex_
    : Struct6
    = Struct6 {
          whous: 2i32,
          lroom: 8i32,
          cella: 9i32,
          mtrol: 10i32,
          maze1: 11i32,
          mgrat: 25i32,
          maz15: 30i32,
          fore1: 31i32,
          fore3: 33i32,
          clear: 36i32,
          reser: 40i32,
          strea: 42i32,
          egypt: 44i32,
          echor: 49i32,
          tshaf: 61i32,
          bshaf: 76i32,
          mmach: 77i32,
          dome: 79i32,
          mtorc: 80i32,
          carou: 83i32,
          riddl: 91i32,
          lld2: 94i32,
          temp1: 96i32,
          temp2: 97i32,
          maint: 100i32,
          blroo: 102i32,
          treas: 103i32,
          rivr1: 107i32,
          rivr2: 108i32,
          rivr3: 109i32,
          mcycl: 101i32,
          rivr4: 112i32,
          rivr5: 113i32,
          fchmp: 114i32,
          falls: 120i32,
          mbarr: 119i32,
          mrain: 121i32,
          pog: 122i32,
          vlbot: 126i32,
          vair1: 127i32,
          vair2: 128i32,
          vair3: 129i32,
          vair4: 130i32,
          ledg2: 131i32,
          ledg3: 132i32,
          ledg4: 133i32,
          msafe: 135i32,
          cager: 140i32,
          caged: 141i32,
          twell: 142i32,
          bwell: 143i32,
          alice: 144i32,
          alism: 145i32,
          alitr: 146i32,
          mtree: 147i32,
          bkent: 148i32,
          bkvw: 151i32,
          bktwi: 153i32,
          bkvau: 154i32,
          bkbox: 155i32,
          crypt: 157i32,
          tstrs: 158i32,
          mrant: 159i32,
          mreye: 160i32,
          mra: 161i32,
          mrb: 162i32,
          mrc: 163i32,
          mrg: 164i32,
          mrd: 165i32,
          fdoor: 166i32,
          mrae: 167i32,
          mrce: 171i32,
          mrcw: 172i32,
          mrge: 173i32,
          mrgw: 174i32,
          mrdw: 176i32,
          inmir: 177i32,
          scorr: 179i32,
          ncorr: 182i32,
          parap: 183i32,
          cell: 184i32,
          pcell: 185i32,
          ncell: 186i32,
          cpant: 188i32,
          cpout: 189i32,
          cpuzz: 190i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
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

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static xsrch_
    : Struct7
    = Struct7 {
          xmin: 1024i32,
          xmax: 16384i32,
          xdown: 10240i32,
          xup: 9216i32,
          xnorth: 1024i32,
          xsouth: 5120i32,
          xenter: 13312i32,
          xexit: 14336i32,
          xeast: 3072i32,
          xwest: 7168i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct8 {
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

impl Clone for Struct8 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut objcts_
    : Struct8
    = Struct8 {
          olnt: 0i32,
          odesc1: [0i32; 220],
          odesc2: [0i32; 220],
          odesco: [0i32; 220],
          oactio: [0i32; 220],
          oflag1: [0i32; 220],
          oflag2: [0i32; 220],
          ofval: [0i32; 220],
          otval: [0i32; 220],
          osize: [0i32; 220],
          ocapac: [0i32; 220],
          oroom: [0i32; 220],
          oadv: [0i32; 220],
          ocan: [0i32; 220],
          oread: [0i32; 220]
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct9 {
    pub r2lnt : i32,
    pub oroom2 : [i32; 20],
    pub rroom2 : [i32; 20],
}

impl Clone for Struct9 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut oroom2_
    : Struct9
    = Struct9 { r2lnt: 0i32, oroom2: [0i32; 20], rroom2: [0i32; 20] };

#[derive(Copy)]
#[repr(C)]
pub struct Struct10 {
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

impl Clone for Struct10 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static oindex_
    : Struct10
    = Struct10 {
          garli: 2i32,
          food: 3i32,
          gunk: 4i32,
          coal: 5i32,
          machi: 7i32,
          diamo: 8i32,
          tcase: 9i32,
          bottl: 10i32,
          water: 11i32,
          rope: 12i32,
          knife: 13i32,
          sword: 14i32,
          lamp: 15i32,
          blamp: 16i32,
          rug: 17i32,
          leave: 18i32,
          troll: 19i32,
          axe: 20i32,
          rknif: 21i32,
          keys: 23i32,
          ice: 30i32,
          bar: 26i32,
          coffi: 33i32,
          torch: 34i32,
          tbask: 35i32,
          fbask: 36i32,
          irbox: 39i32,
          ghost: 42i32,
          trunk: 45i32,
          bell: 46i32,
          book: 47i32,
          candl: 48i32,
          match_: 51i32,
          tube: 54i32,
          putty: 55i32,
          wrenc: 56i32,
          screw: 57i32,
          cyclo: 58i32,
          chali: 59i32,
          thief: 61i32,
          still: 62i32,
          windo: 63i32,
          grate: 65i32,
          door: 66i32,
          hpole: 71i32,
          leak: 78i32,
          rbutt: 79i32,
          raili: 75i32,
          pot: 85i32,
          statu: 86i32,
          iboat: 87i32,
          dboat: 88i32,
          pump: 89i32,
          rboat: 90i32,
          stick: 92i32,
          buoy: 94i32,
          shove: 96i32,
          ballo: 98i32,
          recep: 99i32,
          guano: 97i32,
          brope: 101i32,
          hook1: 102i32,
          hook2: 103i32,
          safe: 105i32,
          sslot: 107i32,
          brick: 109i32,
          fuse: 110i32,
          gnome: 111i32,
          blabe: 112i32,
          dball: 113i32,
          tomb: 119i32,
          lcase: 123i32,
          cage: 124i32,
          rcage: 125i32,
          spher: 126i32,
          sqbut: 127i32,
          flask: 132i32,
          pool: 133i32,
          saffr: 134i32,
          bucke: 137i32,
          ecake: 138i32,
          orice: 139i32,
          rdice: 140i32,
          blice: 141i32,
          robot: 142i32,
          ftree: 145i32,
          bills: 148i32,
          portr: 149i32,
          scol: 151i32,
          zgnom: 152i32,
          egg: 154i32,
          begg: 155i32,
          baubl: 156i32,
          canar: 157i32,
          bcana: 158i32,
          ylwal: 159i32,
          rdwal: 161i32,
          pindr: 164i32,
          rbeam: 171i32,
          odoor: 172i32,
          qdoor: 173i32,
          cdoor: 175i32,
          num1: 178i32,
          num8: 185i32,
          warni: 186i32,
          cslit: 187i32,
          gcard: 188i32,
          stldr: 189i32,
          hands: 200i32,
          wall: 198i32,
          lungs: 201i32,
          sailo: 196i32,
          aviat: 202i32,
          teeth: 197i32,
          itobj: 192i32,
          every: 194i32,
          valua: 195i32,
          oplay: 193i32,
          wnort: 205i32,
          gwate: 209i32,
          master: 215i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct11 {
    pub clnt : i32,
    pub ctick : [i32; 25],
    pub cactio : [i32; 25],
    pub cflag : [i32; 25],
}

impl Clone for Struct11 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut cevent_
    : Struct11
    = Struct11 {
          clnt: 0i32,
          ctick: [0i32; 25],
          cactio: [0i32; 25],
          cflag: [0i32; 25]
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct12 {
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

impl Clone for Struct12 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static cindex_
    : Struct12
    = Struct12 {
          cevcur: 1i32,
          cevmnt: 2i32,
          cevlnt: 3i32,
          cevmat: 4i32,
          cevcnd: 5i32,
          cevbal: 6i32,
          cevbrn: 7i32,
          cevfus: 8i32,
          cevled: 9i32,
          cevsaf: 10i32,
          cevvlg: 11i32,
          cevgno: 12i32,
          cevbuc: 13i32,
          cevsph: 14i32,
          cevegh: 15i32,
          cevfor: 16i32,
          cevscl: 17i32,
          cevzgi: 18i32,
          cevzgo: 19i32,
          cevste: 20i32,
          cevmrs: 21i32,
          cevpin: 22i32,
          cevinq: 23i32,
          cevfol: 24i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct13 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct13 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut advs_
    : Struct13
    = Struct13 {
          alnt: 0i32,
          aroom: [0i32; 4],
          ascore: [0i32; 4],
          avehic: [0i32; 4],
          aobj: [0i32; 4],
          aactio: [0i32; 4],
          astren: [0i32; 4],
          aflag: [0i32; 4]
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct14 {
    pub astag : i32,
}

impl Clone for Struct14 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static aflags_ : Struct14 = Struct14 { astag: 32768i32 };

#[derive(Copy)]
#[repr(C)]
pub struct Struct15 {
    pub player : i32,
    pub arobot : i32,
    pub amastr : i32,
}

impl Clone for Struct15 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static aindex_
    : Struct15
    = Struct15 { player: 1i32, arobot: 2i32, amastr: 3i32 };

#[derive(Copy)]
#[repr(C)]
pub struct Struct16 {
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

impl Clone for Struct16 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static vindex_
    : Struct16
    = Struct16 {
          cintw: 1i32,
          deadxw: 2i32,
          frstqw: 3i32,
          inxw: 4i32,
          outxw: 5i32,
          walkiw: 6i32,
          fightw: 7i32,
          foow: 8i32,
          meltw: 101i32,
          readw: 100i32,
          inflaw: 102i32,
          deflaw: 103i32,
          alarmw: 104i32,
          exorcw: 105i32,
          plugw: 106i32,
          kickw: 107i32,
          wavew: 108i32,
          raisew: 109i32,
          lowerw: 110i32,
          rubw: 111i32,
          pushw: 112i32,
          untiew: 113i32,
          tiew: 114i32,
          tieupw: 115i32,
          turnw: 116i32,
          breatw: 117i32,
          knockw: 118i32,
          lookw: 119i32,
          examiw: 120i32,
          shakew: 121i32,
          movew: 122i32,
          trnonw: 123i32,
          trnofw: 124i32,
          openw: 125i32,
          closew: 126i32,
          findw: 127i32,
          waitw: 128i32,
          spinw: 129i32,
          boardw: 130i32,
          unboaw: 131i32,
          takew: 132i32,
          invenw: 133i32,
          fillw: 134i32,
          eatw: 135i32,
          drinkw: 136i32,
          burnw: 137i32,
          mungw: 138i32,
          killw: 139i32,
          attacw: 141i32,
          swingw: 140i32,
          walkw: 142i32,
          tellw: 143i32,
          putw: 144i32,
          dropw: 145i32,
          givew: 146i32,
          pourw: 147i32,
          throww: 148i32,
          digw: 89i32,
          leapw: 91i32,
          stayw: 73i32,
          follow: 85i32,
          hellow: 151i32,
          lookiw: 152i32,
          lookuw: 153i32,
          pumpw: 154i32,
          windw: 155i32,
          clmbw: 156i32,
          clmbuw: 157i32,
          clmbdw: 158i32,
          trntow: 159i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct17 {
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

impl Clone for Struct17 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut findex_
    : Struct17
    = Struct17 {
          trollf: 0i32,
          cagesf: 0i32,
          bucktf: 0i32,
          caroff: 0i32,
          carozf: 0i32,
          lwtidf: 0i32,
          domef: 0i32,
          glacrf: 0i32,
          echof: 0i32,
          riddlf: 0i32,
          lldf: 0i32,
          cyclof: 0i32,
          magicf: 0i32,
          litldf: 0i32,
          safef: 0i32,
          gnomef: 0i32,
          gnodrf: 0i32,
          mirrmf: 0i32,
          egyptf: 0i32,
          onpolf: 0i32,
          blabf: 0i32,
          brieff: 0i32,
          superf: 0i32,
          buoyf: 0i32,
          grunlf: 0i32,
          gatef: 0i32,
          rainbf: 0i32,
          cagetf: 0i32,
          empthf: 0i32,
          deflaf: 0i32,
          glacmf: 0i32,
          frobzf: 0i32,
          endgmf: 0i32,
          badlkf: 0i32,
          thfenf: 0i32,
          singsf: 0i32,
          mrpshf: 0i32,
          mropnf: 0i32,
          wdopnf: 0i32,
          mr1f: 0i32,
          mr2f: 0i32,
          inqstf: 0i32,
          follwf: 0i32,
          spellf: 0i32,
          cpoutf: 0i32,
          cpushf: 0i32,
          btief: 0i32,
          binff: 0i32,
          rvmnt: 0i32,
          rvclr: 0i32,
          rvcyc: 0i32,
          rvsnd: 0i32,
          rvgua: 0i32,
          orrug: 0i32,
          orcand: 0i32,
          ormtch: 0i32,
          orlamp: 0i32,
          mdir: 0i32,
          mloc: 0i32,
          poleuf: 0i32,
          quesno: 0i32,
          nqatt: 0i32,
          corrct: 0i32,
          lcell: 0i32,
          pnumb: 0i32,
          acell: 0i32,
          dcell: 0i32,
          cphere: 0i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct18 {
    pub dbgflg : i32,
    pub prsflg : i32,
    pub gdtflg : i32,
}

impl Clone for Struct18 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut debug_
    : Struct18
    = Struct18 { dbgflg: 0i32, prsflg: 0i32, gdtflg: 0i32 };

#[derive(Copy)]
#[repr(C)]
pub struct Struct19 {
    pub thfpos : i32,
    pub thfflg : i32,
    pub thfact : i32,
    pub swdact : i32,
    pub swdsta : i32,
}

impl Clone for Struct19 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut hack_
    : Struct19
    = Struct19 {
          thfpos: 0i32,
          thfflg: 0i32,
          thfact: 0i32,
          swdact: 0i32,
          swdsta: 0i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct20 {
    pub vlnt : i32,
    pub villns : [i32; 4],
    pub vprob : [i32; 4],
    pub vopps : [i32; 4],
    pub vbest : [i32; 4],
    pub vmelee : [i32; 4],
}

impl Clone for Struct20 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut vill_
    : Struct20
    = Struct20 {
          vlnt: 0i32,
          villns: [0i32; 4],
          vprob: [0i32; 4],
          vopps: [0i32; 4],
          vbest: [0i32; 4],
          vmelee: [0i32; 4]
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct21 {
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

impl Clone for Struct21 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut state_
    : Struct21
    = Struct21 {
          moves: 0i32,
          deaths: 0i32,
          rwscor: 0i32,
          mxscor: 0i32,
          mxload: 0i32,
          ltshft: 0i32,
          bloc: 0i32,
          mungrm: 0i32,
          hs: 0i32,
          egscor: 0i32,
          egmxsc: 0i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct22 {
    pub xtype : i32,
    pub xroom1 : i32,
    pub xstrng : i32,
    pub xactio : i32,
    pub xobj : i32,
}

impl Clone for Struct22 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut curxt_
    : Struct22
    = Struct22 {
          xtype: 0i32,
          xroom1: 0i32,
          xstrng: 0i32,
          xactio: 0i32,
          xobj: 0i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct23 {
    pub xrmask : i32,
    pub xdmask : i32,
    pub xfmask : i32,
    pub xfshft : i32,
    pub xashft : i32,
    pub xelnt : [i32; 4],
    pub xnorm : i32,
    pub xno : i32,
    pub xcond : i32,
    pub xdoor : i32,
    pub xlflag : i32,
}

impl Clone for Struct23 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static xpars_
    : Struct23
    = Struct23 {
          xrmask: 255i32,
          xdmask: 31744i32,
          xfmask: 3i32,
          xfshft: 256i32,
          xashft: 256i32,
          xelnt: [ 1i32, 2i32, 3i32, 3i32 ],
          xnorm: 1i32,
          xno: 2i32,
          xcond: 3i32,
          xdoor: 4i32,
          xlflag: 32768i32
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct24 {
    pub mbase : i32,
    pub strbit : i32,
}

impl Clone for Struct24 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut star_
    : Struct24
    = Struct24 { mbase: 0i32, strbit: 0i32 };

#[derive(Copy)]
#[repr(C)]
pub struct Struct25 {
    pub inlnt : i32,
    pub inbuf : [u8; 78],
}

impl Clone for Struct25 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut input_
    : Struct25
    = Struct25 { inlnt: 0i32, inbuf: [0u8; 78] };

#[derive(Copy)]
#[repr(C)]
pub struct Struct26 {
    pub fromdr : i32,
    pub scolrm : i32,
    pub scolac : i32,
    pub scoldr : [i32; 8],
    pub scolwl : [i32; 12],
}

impl Clone for Struct26 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut screen_
    : Struct26
    = Struct26 {
          fromdr: 0i32,
          scolrm: 0i32,
          scolac: 0i32,
          scoldr: [   1024i32,
                      153i32,
                      5120i32,
                      154i32,
                      3072i32,
                      152i32,
                      7168i32,
                      151i32
                  ],
          scolwl: [   151i32,
                      207i32,
                      3072i32,
                      152i32,
                      208i32,
                      7168i32,
                      153i32,
                      206i32,
                      5120i32,
                      154i32,
                      205i32,
                      1024i32
                  ]
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct27 {
    pub mlnt : i32,
    pub mrloc : i32,
    pub rtext : [i32; 1050],
}

impl Clone for Struct27 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut rmsg_
    : Struct27
    = Struct27 { mlnt: 0i32, mrloc: 0i32, rtext: [0i32; 1050] };

#[derive(Copy)]
#[repr(C)]
pub struct Struct28 {
    pub vmaj : i32,
    pub vmin : i32,
    pub vedit : i32,
}

impl Clone for Struct28 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static vers_
    : Struct28
    = Struct28 { vmaj: 2i32, vmin: 7i32, vedit: b'A' as (i32) };

#[derive(Copy)]
#[repr(C)]
pub struct Struct29 {
    pub pltime : i32,
    pub shour : i32,
    pub smin : i32,
    pub ssec : i32,
}

impl Clone for Struct29 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut time_
    : Struct29
    = Struct29 { pltime: 0i32, shour: 0i32, smin: 0i32, ssec: 0i32 };

#[derive(Copy)]
#[repr(C)]
pub struct Struct30 {
    pub hfactr : i32,
}

impl Clone for Struct30 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static hyper_ : Struct30 = Struct30 { hfactr: 500i32 };

#[derive(Copy)]
#[repr(C)]
pub struct Struct31 {
    pub xlnt : i32,
    pub travel : [i32; 900],
}

impl Clone for Struct31 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut exits_
    : Struct31
    = Struct31 { xlnt: 0i32, travel: [0i32; 900] };

#[derive(Copy)]
#[repr(C)]
pub struct Struct32 {
    pub cpdr : [i32; 16],
    pub cpwl : [i32; 8],
    pub cpvec : [i32; 64],
}

impl Clone for Struct32 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static mut puzzle_
    : Struct32
    = Struct32 {
          cpdr: [   1024i32,
                    -8i32,
                    2048i32,
                    -7i32,
                    3072i32,
                    1i32,
                    4096i32,
                    9i32,
                    5120i32,
                    8i32,
                    6144i32,
                    7i32,
                    7168i32,
                    -1i32,
                    8192i32,
                    -9i32
                ],
          cpwl: [ 205i32, -8i32, 206i32, 8i32, 207i32, 1i32, 208i32, -1i32 ],
          cpvec: [   1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     0i32,
                     -1i32,
                     0i32,
                     0i32,
                     -1i32,
                     0i32,
                     1i32,
                     1i32,
                     -1i32,
                     0i32,
                     1i32,
                     0i32,
                     -2i32,
                     0i32,
                     1i32,
                     1i32,
                     0i32,
                     0i32,
                     0i32,
                     0i32,
                     1i32,
                     0i32,
                     1i32,
                     1i32,
                     -3i32,
                     0i32,
                     0i32,
                     -1i32,
                     -1i32,
                     0i32,
                     1i32,
                     1i32,
                     0i32,
                     0i32,
                     -1i32,
                     0i32,
                     0i32,
                     0i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     0i32,
                     0i32,
                     0i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32,
                     1i32
                 ]
      };

#[derive(Copy)]
#[repr(C)]
pub struct Struct33 {
    pub batdrp : [i32; 9],
}

impl Clone for Struct33 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub static bats_
    : Struct33
    = Struct33 {
          batdrp: [   66i32,
                      67i32,
                      68i32,
                      69i32,
                      70i32,
                      71i32,
                      72i32,
                      65i32,
                      73i32
                  ]
      };

fn main() {
    use ::std::os::unix::ffi::OsStringExt;
    let mut argv_storage
        = ::std::env::args_os().map(
              |str| {
                        let mut vec = str.into_vec();
                        vec.push(b'\0');
                        vec
                    }
          ).collect::<Vec<_>>(
          );
    let mut argv
        = argv_storage.iter_mut().map(|vec| vec.as_mut_ptr()).chain(
              Some(::std::ptr::null_mut())
          ).collect::<Vec<_>>(
          );
    let ret
        = unsafe {
              _c_main(argv_storage.len() as (i32),argv.as_mut_ptr())
          };
    ::std::process::exit(ret);
}

#[no_mangle]
pub unsafe extern fn _c_main(mut argc : i32, mut argv : *mut *mut u8) -> i32 {
    if init_() != 0 {
        game_();
    }
    exit_();
    0
}
