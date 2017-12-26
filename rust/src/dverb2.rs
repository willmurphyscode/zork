extern {
    static mut advs_ : Struct10;
    static aindex_ : Struct15;
    fn bug_(arg1 : i32, arg2 : i32);
    static mut cevent_ : Struct13;
    fn cpgoto_(arg1 : i32);
    static mut curxt_ : Struct17;
    static mut exits_ : Struct22;
    fn fclose(__stream : *mut _IO_FILE) -> i32;
    static mut findex_ : Struct11;
    fn findxt_(arg1 : i32, arg2 : i32) -> i32;
    fn fopen(
        __filename : *const u8, __modes : *const u8
    ) -> *mut _IO_FILE;
    fn fread(
        __ptr : *mut ::std::os::raw::c_void,
        __size : usize,
        __n : usize,
        __stream : *mut _IO_FILE
    ) -> usize;
    fn fwrite(
        __ptr : *const ::std::os::raw::c_void,
        __size : usize,
        __n : usize,
        __s : *mut _IO_FILE
    ) -> usize;
    fn gttime_(arg1 : *mut i32);
    static mut hack_ : Struct4;
    fn jigsup_(arg1 : i32);
    fn lit_(arg1 : i32) -> i32;
    fn moveto_(arg1 : i32, arg2 : i32) -> i32;
    fn mrhere_(arg1 : i32) -> i32;
    static mut objcts_ : Struct8;
    static oindex_ : Struct20;
    static mut play_ : Struct3;
    fn prob_(arg1 : i32, arg2 : i32) -> i32;
    static mut prsvec_ : Struct1;
    static mut puzzle_ : Struct5;
    static rindex_ : Struct23;
    fn rmdesc_(arg1 : i32) -> i32;
    fn rnd_(arg1 : i32) -> i32;
    static mut rooms_ : Struct9;
    fn rspeak_(arg1 : i32);
    fn rspsub_(arg1 : i32, arg2 : i32);
    static mut screen_ : Struct7;
    static mut state_ : Struct6;
    static mut time_ : Struct14;
    static vers_ : Struct2;
    static mut vill_ : Struct12;
    static xpars_ : Struct21;
    static xsrch_ : Struct18;
}

enum _IO_FILE {
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

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub vmaj : i32,
    pub vmin : i32,
    pub vedit : i32,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub winner : i32,
    pub here : i32,
    pub telflg : i32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub thfpos : i32,
    pub thfflg : i32,
    pub thfact : i32,
    pub swdact : i32,
    pub swdsta : i32,
}

impl Clone for Struct4 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct5 {
    pub cpdr : [i32; 16],
    pub cpwl : [i32; 8],
    pub cpvec : [i32; 64],
}

impl Clone for Struct5 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct6 {
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

impl Clone for Struct6 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
    pub fromdr : i32,
    pub scolrm : i32,
    pub scolac : i32,
    pub scoldr : [i32; 8],
    pub scolwl : [i32; 12],
}

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

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

#[derive(Copy)]
#[repr(C)]
pub struct Struct9 {
    pub rlnt : i32,
    pub rdesc1 : [i32; 200],
    pub rdesc2 : [i32; 200],
    pub rexit : [i32; 200],
    pub ractio : [i32; 200],
    pub rval : [i32; 200],
    pub rflag : [i32; 200],
}

impl Clone for Struct9 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct10 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
}

impl Clone for Struct10 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct11 {
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

impl Clone for Struct11 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct12 {
    pub vlnt : i32,
    pub villns : [i32; 4],
    pub vprob : [i32; 4],
    pub vopps : [i32; 4],
    pub vbest : [i32; 4],
    pub vmelee : [i32; 4],
}

impl Clone for Struct12 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct13 {
    pub clnt : i32,
    pub ctick : [i32; 25],
    pub cactio : [i32; 25],
    pub cflag : [i32; 25],
}

impl Clone for Struct13 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn savegm_() {
    let mut i : i32;
    let mut e : *mut _IO_FILE;
    prsvec_.prswon = 0i32;
    if !({
             e = fopen((*b"dsave.dat\0").as_ptr(),(*b"w\0").as_ptr());
             e
         } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE)) {
        gttime_(&mut i as (*mut i32));
        fwrite(
            &vers_.vmaj as (*const i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &vers_.vmin as (*const i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &vers_.vedit as (*const i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut play_.winner as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut play_.here as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut hack_.thfpos as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut play_.telflg as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut hack_.thfflg as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut hack_.thfact as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut hack_.swdact as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut hack_.swdsta as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut puzzle_.cpvec[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            64usize,
            e
        );
        fwrite(
            &mut i as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.moves as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.deaths as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.rwscor as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.egscor as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.mxload as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.ltshft as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.bloc as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.mungrm as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut state_.hs as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut screen_.fromdr as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut screen_.scolrm as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut screen_.scolac as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fwrite(
            &mut objcts_.odesc1[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.odesc2[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.oflag1[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.oflag2[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.ofval[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.otval[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.osize[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.ocapac[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.oroom[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.oadv[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut objcts_.ocan[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            220usize,
            e
        );
        fwrite(
            &mut rooms_.rval[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            200usize,
            e
        );
        fwrite(
            &mut rooms_.rflag[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            200usize,
            e
        );
        fwrite(
            &mut advs_.aroom[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            4usize,
            e
        );
        fwrite(
            &mut advs_.ascore[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            4usize,
            e
        );
        fwrite(
            &mut advs_.avehic[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            4usize,
            e
        );
        fwrite(
            &mut advs_.astren[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            4usize,
            e
        );
        fwrite(
            &mut advs_.aflag[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            4usize,
            e
        );
        fwrite(
            &mut *(&mut findex_ as (*mut Struct11) as (*mut i32)).offset(
                      0isize
                  ) as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            46usize,
            e
        );
        fwrite(
            &mut *(&mut findex_ as (*mut Struct11) as (*mut i32)).offset(
                      46isize
                  ).offset(
                      0isize
                  ) as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            22usize,
            e
        );
        fwrite(
            &mut vill_.vprob[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            4usize,
            e
        );
        fwrite(
            &mut cevent_.cflag[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            25usize,
            e
        );
        fwrite(
            &mut cevent_.ctick[
                     0usize
                 ] as (*mut i32) as (*const u8) as (*const ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            25usize,
            e
        );
        if !(fclose(e) == -1i32) {
            rspeak_(597i32);
            return;
        }
    }
    rspeak_(598i32);
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct14 {
    pub pltime : i32,
    pub shour : i32,
    pub smin : i32,
    pub ssec : i32,
}

impl Clone for Struct14 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn rstrgm_() {
    let mut i : i32;
    let mut j : i32;
    let mut k : i32;
    let mut e : *mut _IO_FILE;
    prsvec_.prswon = 0i32;
    if {
           e = fopen((*b"dsave.dat\0").as_ptr(),(*b"r\0").as_ptr());
           e
       } == 0i32 as (*mut ::std::os::raw::c_void) as (*mut _IO_FILE) {
        rspeak_(598i32);
    } else {
        fread(
            &mut i as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fread(
            &mut j as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        fread(
            &mut k as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
            ::std::mem::size_of::<i32>(),
            1usize,
            e
        );
        (if (i != vers_.vmaj) as (i32) | (j != vers_.vmin) as (i32) != 0 {
             rspeak_(600i32);
             fclose(e);
         } else {
             fread(
                 &mut play_.winner as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut play_.here as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut hack_.thfpos as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut play_.telflg as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut hack_.thfflg as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut hack_.thfact as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut hack_.swdact as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut hack_.swdsta as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut puzzle_.cpvec[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 64usize,
                 e
             );
             fread(
                 &mut time_.pltime as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.moves as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.deaths as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.rwscor as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.egscor as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.mxload as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.ltshft as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.bloc as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.mungrm as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut state_.hs as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut screen_.fromdr as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut screen_.scolrm as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut screen_.scolac as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 1usize,
                 e
             );
             fread(
                 &mut objcts_.odesc1[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.odesc2[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.oflag1[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.oflag2[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.ofval[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.otval[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.osize[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.ocapac[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.oroom[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.oadv[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut objcts_.ocan[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 220usize,
                 e
             );
             fread(
                 &mut rooms_.rval[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 200usize,
                 e
             );
             fread(
                 &mut rooms_.rflag[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 200usize,
                 e
             );
             fread(
                 &mut advs_.aroom[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 4usize,
                 e
             );
             fread(
                 &mut advs_.ascore[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 4usize,
                 e
             );
             fread(
                 &mut advs_.avehic[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 4usize,
                 e
             );
             fread(
                 &mut advs_.astren[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 4usize,
                 e
             );
             fread(
                 &mut advs_.aflag[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 4usize,
                 e
             );
             fread(
                 &mut *(&mut findex_ as (*mut Struct11) as (*mut i32)).offset(
                           0isize
                       ) as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 46usize,
                 e
             );
             fread(
                 &mut *(&mut findex_ as (*mut Struct11) as (*mut i32)).offset(
                           46isize
                       ).offset(
                           0isize
                       ) as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 22usize,
                 e
             );
             fread(
                 &mut vill_.vprob[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 4usize,
                 e
             );
             fread(
                 &mut cevent_.cflag[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 25usize,
                 e
             );
             fread(
                 &mut cevent_.ctick[
                          0usize
                      ] as (*mut i32) as (*mut u8) as (*mut ::std::os::raw::c_void),
                 ::std::mem::size_of::<i32>(),
                 25usize,
                 e
             );
             fclose(e);
             rspeak_(599i32);
         })
    }
}

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

#[derive(Copy)]
#[repr(C)]
pub struct Struct17 {
    pub xtype : i32,
    pub xroom1 : i32,
    pub xstrng : i32,
    pub xactio : i32,
    pub xobj : i32,
}

impl Clone for Struct17 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct18 {
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

impl Clone for Struct18 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn walk_() -> i32 {
    let mut _currentBlock;
    let mut ret_val : i32;
    ret_val = 1i32;
    if play_.winner != aindex_.player || lit_(
                                             play_.here
                                         ) != 0 || prob_(25i32,25i32) != 0 {
        if findxt_(prsvec_.prso,play_.here) != 0 {
            let switch19 = curxt_.xtype;
            if switch19 == 1i32 {
                _currentBlock = 39;
            } else if switch19 == 4i32 {
                if cxappl_(curxt_.xactio) != 0i32 {
                    _currentBlock = 39;
                } else if objcts_.oflag2[
                              (curxt_.xobj - 1i32) as (usize)
                          ] & 8i32 != 0i32 {
                    _currentBlock = 39;
                } else {
                    if curxt_.xstrng == 0i32 {
                        curxt_.xstrng = 525i32;
                    }
                    rspsub_(
                        curxt_.xstrng,
                        objcts_.odesc2[(curxt_.xobj - 1i32) as (usize)]
                    );
                    prsvec_.prscon = 1i32;
                    return ret_val;
                }
            } else {
                if switch19 == 3i32 {
                    _currentBlock = 23;
                } else if switch19 == 2i32 {
                    _currentBlock = 25;
                } else {
                    bug_(9i32,curxt_.xtype);
                    _currentBlock = 23;
                }
                if _currentBlock == 23 {
                    if cxappl_(curxt_.xactio) != 0i32 {
                        _currentBlock = 39;
                    } else if *(&mut findex_ as (*mut Struct11) as (*mut i32)).offset(
                                   (*(&mut curxt_ as (*mut Struct17) as (*mut i32)).offset(
                                         4isize
                                     ) - 1i32) as (isize)
                               ) != 0 {
                        _currentBlock = 39;
                    } else {
                        _currentBlock = 25;
                    }
                }
                if _currentBlock == 39 {
                } else if curxt_.xstrng == 0i32 {
                    _currentBlock = 27;
                } else {
                    rspeak_(curxt_.xstrng);
                    prsvec_.prscon = 1i32;
                    return ret_val;
                }
            }
        } else {
            _currentBlock = 27;
        }
        if _currentBlock == 39 {
        } else {
            curxt_.xstrng = 678i32;
            if prsvec_.prso == xsrch_.xup {
                curxt_.xstrng = 679i32;
            }
            if prsvec_.prso == xsrch_.xdown {
                curxt_.xstrng = 680i32;
            }
            if rooms_.rflag[(play_.here - 1i32) as (usize)] & 32i32 != 0i32 {
                curxt_.xstrng = 524i32;
            }
            rspeak_(curxt_.xstrng);
            prsvec_.prscon = 1i32;
            return ret_val;
        }
    } else {
        if findxt_(prsvec_.prso,play_.here) == 0 {
            _currentBlock = 15;
        } else {
            let switch16 = curxt_.xtype;
            if switch16 == 4i32 {
                if !(cxappl_(curxt_.xactio) != 0i32) {
                    if !(objcts_.oflag2[
                             (curxt_.xobj - 1i32) as (usize)
                         ] & 8i32 != 0i32) {
                        jigsup_(523i32);
                        return ret_val;
                    }
                }
            } else {
                if switch16 == 3i32 {
                    _currentBlock = 8;
                } else if switch16 == 2i32 {
                    _currentBlock = 10;
                } else if switch16 == 1i32 {
                    _currentBlock = 14;
                } else {
                    bug_(9i32,curxt_.xtype);
                    _currentBlock = 8;
                }
                if _currentBlock == 14 {
                } else {
                    if _currentBlock == 8 {
                        if cxappl_(curxt_.xactio) != 0i32 {
                            _currentBlock = 14;
                        } else if *(&mut findex_ as (*mut Struct11) as (*mut i32)).offset(
                                       (*(&mut curxt_ as (*mut Struct17) as (*mut i32)).offset(
                                             4isize
                                         ) - 1i32) as (isize)
                                   ) != 0 {
                            _currentBlock = 14;
                        } else {
                            _currentBlock = 10;
                        }
                    }
                    if _currentBlock == 14 {
                    } else {
                        jigsup_(523i32);
                        return ret_val;
                    }
                }
            }
            if lit_(curxt_.xroom1) != 0 {
                _currentBlock = 39;
            } else {
                _currentBlock = 15;
            }
        }
        if _currentBlock == 39 {
        } else {
            jigsup_(522i32);
            return ret_val;
        }
    }
    ret_val = moveto_(curxt_.xroom1,play_.winner);
    if ret_val != 0 {
        ret_val = rmdesc_(0i32);
    }
    ret_val
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct20 {
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

impl Clone for Struct20 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct21 {
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

impl Clone for Struct21 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct22 {
    pub xlnt : i32,
    pub travel : [i32; 900],
}

impl Clone for Struct22 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct23 {
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

impl Clone for Struct23 {
    fn clone(&self) -> Self { *self }
}

unsafe extern fn cxappl_(mut ri : i32) -> i32 {
    let mut _currentBlock;
    let mut ret_val : i32;
    let mut i__1 : i32;
    let mut i : i32;
    let mut j : i32;
    let mut k : i32;
    let mut nxt : i32;
    let mut ldir : i32;
    ret_val = 0i32;
    if ri == 0i32 {
        ret_val
    } else if ri == 14i32 {
        findex_.frobzf = 0i32;
        (if prsvec_.prso != xsrch_.xup {
             (if findex_.cphere != 52i32 || prsvec_.prso != xsrch_.xwest || findex_.cpoutf == 0 {
                  i = 1i32;
                  'loop80: loop {
                      if !(i <= 16i32) {
                          _currentBlock = 81;
                          break;
                      }
                      if prsvec_.prso == puzzle_.cpdr[(i - 1i32) as (usize)] {
                          _currentBlock = 84;
                          break;
                      }
                      i = i + 2i32;
                  }
                  (if _currentBlock == 81 {
                       ret_val
                   } else {
                       j = puzzle_.cpdr[i as (usize)];
                       nxt = findex_.cphere + j;
                       k = 8i32;
                       if j < 0i32 {
                           k = -8i32;
                       }
                       (if (if j >= 0i32 { j } else { -j } == 1i32 || if j >= 0i32 {
                                                                          j
                                                                      } else {
                                                                          -j
                                                                      } == 8i32 || (puzzle_.cpvec[
                                                                                        (findex_.cphere + k - 1i32) as (usize)
                                                                                    ] == 0i32 || puzzle_.cpvec[
                                                                                                     (nxt - k - 1i32) as (usize)
                                                                                                 ] == 0i32)) && (puzzle_.cpvec[
                                                                                                                     (nxt - 1i32) as (usize)
                                                                                                                 ] == 0i32) {
                            cpgoto_(nxt);
                            curxt_.xroom1 = rindex_.cpuzz;
                            ret_val = curxt_.xroom1;
                            ret_val
                        } else {
                            ret_val
                        })
                   })
              } else {
                  findex_.frobzf = 1i32;
                  ret_val
              })
         } else if findex_.cphere != 10i32 {
             ret_val
         } else {
             curxt_.xstrng = 881i32;
             (if puzzle_.cpvec[findex_.cphere as (usize)] != -2i32 {
                  ret_val
              } else {
                  rspeak_(882i32);
                  findex_.frobzf = 1i32;
                  ret_val
              })
         })
    } else if ri == 13i32 {
        findex_.cphere = 52i32;
        ret_val
    } else if ri == 12i32 {
        findex_.frobzf = 1i32;
        findex_.cphere = 10i32;
        ret_val
    } else if ri == 11i32 {
        if findex_.lcell != 4i32 {
            curxt_.xstrng = 678i32;
        }
        ret_val
    } else if ri == 10i32 {
        findex_.frobzf = 0i32;
        ldir = (prsvec_.prso - xsrch_.xnorth) / xsrch_.xnorth * 45i32;
        (if findex_.mropnf == 0 || (findex_.mdir + 270i32) % 360i32 != ldir && (prsvec_.prso != xsrch_.xexit) {
             (if findex_.wdopnf == 0 || (findex_.mdir + 180i32) % 360i32 != ldir && (prsvec_.prso != xsrch_.xexit) {
                  ret_val
              } else {
                  curxt_.xroom1 = findex_.mloc + 1i32;
                  if findex_.mdir == 0i32 {
                      curxt_.xroom1 = findex_.mloc - 1i32;
                  }
                  rspeak_(818i32);
                  findex_.wdopnf = 0i32;
                  ret_val = curxt_.xroom1;
                  ret_val
              })
         } else {
             curxt_.xroom1 = (findex_.mloc - rindex_.mra << 1i32) + rindex_.mrae + 1i32 - findex_.mdir / 180i32;
             if !(findex_.mdir % 180i32 == 0i32) {
                 curxt_.xroom1 = findex_.mloc + 1i32;
                 if findex_.mdir > 180i32 {
                     curxt_.xroom1 = findex_.mloc - 1i32;
                 }
             }
             ret_val = curxt_.xroom1;
             ret_val
         })
    } else if ri == 9i32 {
        (if mrhere_(play_.here) != 1i32 {
             findex_.frobzf = 0i32;
             curxt_.xstrng = 817i32;
             ret_val
         } else {
             if findex_.mr1f != 0 {
                 curxt_.xstrng = 805i32;
             }
             findex_.frobzf = findex_.mropnf;
             ret_val
         })
    } else if ri == 8i32 {
        findex_.frobzf = 0i32;
        if !(findex_.mloc != curxt_.xroom1) {
            if prsvec_.prso == xsrch_.xnorth || prsvec_.prso == xsrch_.xsouth {
                curxt_.xstrng = 814i32;
                if findex_.mdir % 180i32 == 0i32 {
                    return ret_val;
                } else {
                    _currentBlock = 44;
                }
            } else if findex_.mdir % 180i32 != 0i32 {
                _currentBlock = 44;
            } else {
                curxt_.xroom1 = (curxt_.xroom1 - rindex_.mra << 1i32) + rindex_.mrae;
                if prsvec_.prso > xsrch_.xsouth {
                    curxt_.xroom1 = curxt_.xroom1 + 1;
                    _currentBlock = 50;
                } else {
                    _currentBlock = 50;
                }
            }
            if _currentBlock == 50 {
            } else {
                ldir = findex_.mdir;
                if prsvec_.prso == xsrch_.xsouth {
                    ldir = 180i32;
                }
                curxt_.xstrng = 815i32;
                if ldir > 180i32 && (findex_.mr1f == 0) || ldir < 180i32 && (findex_.mr2f == 0) {
                    curxt_.xstrng = 816i32;
                }
                return ret_val;
            }
        }
        ret_val = curxt_.xroom1;
        ret_val
    } else if ri == 7i32 {
        findex_.frobzf = (objcts_.oroom[
                              (oindex_.bills - 1i32) as (usize)
                          ] != 0i32) as (i32) & (objcts_.oroom[
                                                     (oindex_.portr - 1i32) as (usize)
                                                 ] != 0i32) as (i32);
        ret_val
    } else {
        if ri == 6i32 {
            if findex_.caroff != 0 {
                _currentBlock = 35;
            } else {
                findex_.frobzf = 1i32;
                return ret_val;
            }
        } else if ri == 5i32 {
            _currentBlock = 36;
        } else {
            if ri == 4i32 {
                if findex_.caroff == 0 {
                    findex_.frobzf = 0i32;
                    return ret_val;
                }
            } else if ri == 3i32 {
                findex_.litldf = 0i32;
                j = 0i32;
                i__1 = objcts_.olnt;
                i = 1i32;
                'loop20: loop {
                    if !(i <= i__1) {
                        break;
                    }
                    if objcts_.oadv[(i - 1i32) as (usize)] == play_.winner {
                        j = j + 1;
                    }
                    i = i + 1;
                }
                if j > 2i32 {
                    return ret_val;
                } else {
                    curxt_.xstrng = 446i32;
                    if objcts_.oadv[(oindex_.lamp - 1i32) as (usize)] != play_.winner {
                        return ret_val;
                    } else {
                        findex_.litldf = 1i32;
                        if objcts_.oflag2[
                               (oindex_.door - 1i32) as (usize)
                           ] & 8i32 == 0i32 {
                            let _rhs = !4i32;
                            let _lhs = &mut objcts_.oflag2[(oindex_.door - 1i32) as (usize)];
                            *_lhs = *_lhs & _rhs;
                        }
                        return ret_val;
                    }
                }
            } else if ri == 2i32 {
                if findex_.caroff != 0 {
                    return ret_val;
                }
            } else {
                if !(ri == 1i32) {
                    bug_(5i32,ri);
                }
                findex_.egyptf = (objcts_.oadv[
                                      (oindex_.coffi - 1i32) as (usize)
                                  ] != play_.winner) as (i32);
                return ret_val;
            }
            _currentBlock = 35;
        }
        if _currentBlock == 35 {
            rspeak_(121i32);
        }
        i = xpars_.xelnt[(xpars_.xcond - 1i32) as (usize)] * rnd_(8i32);
        curxt_.xroom1 = exits_.travel[
                            (rooms_.rexit[
                                 (play_.here - 1i32) as (usize)
                             ] + i - 1i32) as (usize)
                        ] & xpars_.xrmask;
        ret_val = curxt_.xroom1;
        ret_val
    }
}
