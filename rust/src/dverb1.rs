extern {
    static mut advs_ : Struct4;
    fn lit_(arg1 : i32) -> i32;
    fn newsta_(
        arg1 : i32, arg2 : i32, arg3 : i32, arg4 : i32, arg5 : i32
    );
    fn oappli_(arg1 : i32, arg2 : i32) -> i32;
    fn objact_() -> i32;
    static mut objcts_ : Struct1;
    static mut play_ : Struct5;
    static mut prsvec_ : Struct2;
    fn qhere_(arg1 : i32, arg2 : i32) -> i32;
    static rindex_ : Struct7;
    fn rnd_(arg1 : i32) -> i32;
    fn rspeak_(arg1 : i32);
    fn rspsb2_(arg1 : i32, arg2 : i32, arg3 : i32);
    fn rspsub_(arg1 : i32, arg2 : i32);
    fn scrupd_(arg1 : i32);
    static mut star_ : Struct3;
    static mut state_ : Struct6;
    static vindex_ : Struct8;
    fn weight_(arg1 : i32, arg2 : i32, arg3 : i32) -> i32;
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

#[derive(Copy)]
#[repr(C)]
pub struct Struct2 {
    pub prsa : i32,
    pub prsi : i32,
    pub prso : i32,
    pub prswon : i32,
    pub prscon : i32,
}

impl Clone for Struct2 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct3 {
    pub mbase : i32,
    pub strbit : i32,
}

impl Clone for Struct3 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct4 {
    pub alnt : i32,
    pub aroom : [i32; 4],
    pub ascore : [i32; 4],
    pub avehic : [i32; 4],
    pub aobj : [i32; 4],
    pub aactio : [i32; 4],
    pub astren : [i32; 4],
    pub aflag : [i32; 4],
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

#[no_mangle]
pub unsafe extern fn take_(mut flg : i32) -> i32 {
    let mut i__1 : i32;
    let mut ret_val : i32;
    let mut oa : i32;
    let mut x : i32;
    ret_val = 0i32;
    oa = objcts_.oactio[(prsvec_.prso - 1i32) as (usize)];
    if prsvec_.prso <= star_.strbit {
        x = objcts_.ocan[(prsvec_.prso - 1i32) as (usize)];
        (if prsvec_.prso != advs_.avehic[
                                (play_.winner - 1i32) as (usize)
                            ] {
             (if objcts_.oflag1[
                     (prsvec_.prso - 1i32) as (usize)
                 ] & 8192i32 != 0i32 {
                  (if x != 0i32 || qhere_(prsvec_.prso,play_.here) != 0 {
                       (if x != 0i32 && (objcts_.oadv[
                                             (x - 1i32) as (usize)
                                         ] == play_.winner) || weight_(
                                                                   0i32,
                                                                   prsvec_.prso,
                                                                   play_.winner
                                                               ) + objcts_.osize[
                                                                       (prsvec_.prso - 1i32) as (usize)
                                                                   ] <= state_.mxload {
                            ret_val = 1i32;
                            (if oappli_(oa,0i32) != 0 {
                                 ret_val
                             } else {
                                 newsta_(prsvec_.prso,0i32,0i32,0i32,play_.winner);
                                 let _rhs = 4i32;
                                 let _lhs = &mut objcts_.oflag2[(prsvec_.prso - 1i32) as (usize)];
                                 *_lhs = *_lhs | _rhs;
                                 scrupd_(objcts_.ofval[(prsvec_.prso - 1i32) as (usize)]);
                                 objcts_.ofval[(prsvec_.prso - 1i32) as (usize)] = 0i32;
                                 if flg != 0 {
                                     rspeak_(559i32);
                                 }
                                 ret_val
                             })
                        } else {
                            rspeak_(558i32);
                            ret_val
                        })
                   } else {
                       if objcts_.oadv[(prsvec_.prso - 1i32) as (usize)] == play_.winner {
                           rspeak_(557i32);
                       }
                       ret_val
                   })
              } else {
                  if oappli_(oa,0i32) == 0 {
                      i__1 = rnd_(5i32) + 552i32;
                      rspeak_(i__1);
                  }
                  ret_val
              })
         } else {
             rspeak_(672i32);
             ret_val
         })
    } else {
        ret_val = objact_();
        ret_val
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct7 {
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

impl Clone for Struct7 {
    fn clone(&self) -> Self { *self }
}

#[derive(Copy)]
#[repr(C)]
pub struct Struct8 {
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

impl Clone for Struct8 {
    fn clone(&self) -> Self { *self }
}

#[no_mangle]
pub unsafe extern fn drop_(mut z : i32) -> i32 {
    let mut _currentBlock;
    let mut ret_val : i32;
    let mut f : i32;
    let mut i : i32;
    let mut x : i32;
    ret_val = 1i32;
    x = objcts_.ocan[(prsvec_.prso - 1i32) as (usize)];
    if x == 0i32 {
        if objcts_.oadv[(prsvec_.prso - 1i32) as (usize)] != play_.winner {
            _currentBlock = 18;
        } else {
            _currentBlock = 5;
        }
    } else if objcts_.oadv[(x - 1i32) as (usize)] != play_.winner {
        _currentBlock = 18;
    } else if objcts_.oflag2[(x - 1i32) as (usize)] & 8i32 != 0i32 {
        _currentBlock = 5;
    } else {
        rspsub_(525i32,objcts_.odesc2[(x - 1i32) as (usize)]);
        return ret_val;
    }
    if _currentBlock == 5 {
        (if advs_.avehic[(play_.winner - 1i32) as (usize)] == 0i32 {
             newsta_(prsvec_.prso,0i32,play_.here,0i32,0i32);
             if play_.here == rindex_.mtree {
                 newsta_(prsvec_.prso,0i32,rindex_.fore3,0i32,0i32);
             }
             scrupd_(objcts_.ofval[(prsvec_.prso - 1i32) as (usize)]);
             objcts_.ofval[(prsvec_.prso - 1i32) as (usize)] = 0i32;
             let _rhs = 4i32;
             let _lhs = &mut objcts_.oflag2[(prsvec_.prso - 1i32) as (usize)];
             *_lhs = *_lhs | _rhs;
             (if objact_() != 0 {
                  ret_val
              } else {
                  i = 0i32;
                  if prsvec_.prsa == vindex_.dropw {
                      i = 528i32;
                  }
                  if prsvec_.prsa == vindex_.throww {
                      i = 529i32;
                  }
                  if i != 0i32 && (play_.here == rindex_.mtree) {
                      i = 659i32;
                  }
                  rspsub_(i,objcts_.odesc2[(prsvec_.prso - 1i32) as (usize)]);
                  ret_val
              })
         } else {
             prsvec_.prsi = advs_.avehic[(play_.winner - 1i32) as (usize)];
             f = put_(1i32);
             prsvec_.prsi = 0i32;
             ret_val
         })
    } else {
        rspeak_(527i32);
        ret_val
    }
}

#[no_mangle]
pub unsafe extern fn put_(mut flg : i32) -> i32 {
    let mut _currentBlock;
    let mut ret_val : i32;
    let mut j : i32;
    let mut svi : i32;
    let mut svo : i32;
    ret_val = 0i32;
    if prsvec_.prso <= star_.strbit && (prsvec_.prsi <= star_.strbit) {
        (if objcts_.oflag2[
                (prsvec_.prsi - 1i32) as (usize)
            ] & 8i32 != 0i32 || objcts_.oflag1[
                                    (prsvec_.prsi - 1i32) as (usize)
                                ] & 4096i32 + 128i32 != 0i32 || objcts_.oflag2[
                                                                    (prsvec_.prsi - 1i32) as (usize)
                                                                ] & 2i32 != 0i32 {
             (if objcts_.oflag2[
                     (prsvec_.prsi - 1i32) as (usize)
                 ] & 8i32 != 0i32 {
                  (if prsvec_.prso != prsvec_.prsi {
                       (if objcts_.ocan[
                               (prsvec_.prso - 1i32) as (usize)
                           ] != prsvec_.prsi {
                            (if weight_(0i32,prsvec_.prso,0i32) + weight_(
                                                                      0i32,
                                                                      prsvec_.prsi,
                                                                      0i32
                                                                  ) + objcts_.osize[
                                                                          (prsvec_.prso - 1i32) as (usize)
                                                                      ] <= objcts_.ocapac[
                                                                               (prsvec_.prsi - 1i32) as (usize)
                                                                           ] {
                                 j = prsvec_.prso;
                                 'loop15: loop {
                                     if qhere_(j,play_.here) != 0 {
                                         _currentBlock = 21;
                                         break;
                                     }
                                     j = objcts_.ocan[(j - 1i32) as (usize)];
                                     if !(j != 0i32) {
                                         _currentBlock = 17;
                                         break;
                                     }
                                 }
                                 if _currentBlock == 17 {
                                     if !(objcts_.ocan[(prsvec_.prso - 1i32) as (usize)] == 0i32) {
                                         if objcts_.oflag2[
                                                (objcts_.ocan[
                                                     (prsvec_.prso - 1i32) as (usize)
                                                 ] - 1i32) as (usize)
                                            ] & 8i32 != 0i32 {
                                             scrupd_(
                                                 objcts_.ofval[(prsvec_.prso - 1i32) as (usize)]
                                             );
                                             objcts_.ofval[(prsvec_.prso - 1i32) as (usize)] = 0i32;
                                             let _rhs = 4i32;
                                             let _lhs
                                                 = &mut objcts_.oflag2[
                                                            (prsvec_.prso - 1i32) as (usize)
                                                        ];
                                             *_lhs = *_lhs | _rhs;
                                             newsta_(prsvec_.prso,0i32,0i32,0i32,play_.winner);
                                         } else {
                                             rspsub_(
                                                 566i32,
                                                 objcts_.odesc2[(prsvec_.prso - 1i32) as (usize)]
                                             );
                                             return ret_val;
                                         }
                                     }
                                 } else {
                                     svo = prsvec_.prso;
                                     svi = prsvec_.prsi;
                                     prsvec_.prsa = vindex_.takew;
                                     prsvec_.prsi = 0i32;
                                     if take_(0i32) == 0 {
                                         return ret_val;
                                     } else {
                                         prsvec_.prsa = vindex_.putw;
                                         prsvec_.prso = svo;
                                         prsvec_.prsi = svi;
                                     }
                                 }
                                 (if objact_() != 0 {
                                      ret_val
                                  } else {
                                      newsta_(prsvec_.prso,2i32,0i32,prsvec_.prsi,0i32);
                                      ret_val = 1i32;
                                      ret_val
                                  })
                             } else {
                                 rspeak_(565i32);
                                 ret_val
                             })
                        } else {
                            rspsb2_(
                                564i32,
                                objcts_.odesc2[(prsvec_.prso - 1i32) as (usize)],
                                objcts_.odesc2[(prsvec_.prsi - 1i32) as (usize)]
                            );
                            ret_val = 1i32;
                            ret_val
                        })
                   } else {
                       rspeak_(563i32);
                       ret_val
                   })
              } else {
                  rspeak_(562i32);
                  ret_val
              })
         } else {
             rspeak_(561i32);
             ret_val
         })
    } else {
        if objact_() == 0 {
            rspeak_(560i32);
        }
        ret_val = 1i32;
        ret_val
    }
}

#[no_mangle]
pub unsafe extern fn valuac_(mut v : i32) {
    let mut _currentBlock;
    let mut i__1 : i32;
    let mut f : i32;
    let mut i : i32;
    let mut f1 : i32;
    let mut savep : i32;
    let mut saveh : i32;
    f = 1i32;
    i = 579i32;
    if !(lit_(play_.here) == 0) {
        i = 677i32;
        savep = prsvec_.prso;
        saveh = play_.here;
        if prsvec_.prsa != vindex_.takew {
            if prsvec_.prsa != vindex_.dropw {
                if !(prsvec_.prsa != vindex_.putw) {
                    i__1 = objcts_.olnt;
                    prsvec_.prso = 1i32;
                    'loop18: loop {
                        if !(prsvec_.prso <= i__1) {
                            _currentBlock = 23;
                            break;
                        }
                        if !(objcts_.oadv[
                                 (prsvec_.prso - 1i32) as (usize)
                             ] != play_.winner || prsvec_.prso == prsvec_.prsi || savep == v && (objcts_.otval[
                                                                                                     (prsvec_.prso - 1i32) as (usize)
                                                                                                 ] <= 0i32) || objcts_.oflag1[
                                                                                                                   (prsvec_.prso - 1i32) as (usize)
                                                                                                               ] & 32768i32 == 0i32) {
                            f = 0i32;
                            rspsub_(580i32,objcts_.odesc2[(prsvec_.prso - 1i32) as (usize)]);
                            f1 = put_(1i32);
                            if saveh != play_.here {
                                _currentBlock = 21;
                                break;
                            }
                        }
                        prsvec_.prso = prsvec_.prso + 1;
                    }
                    if _currentBlock == 23 {
                    } else {
                        return;
                    }
                }
            } else {
                i__1 = objcts_.olnt;
                prsvec_.prso = 1i32;
                'loop11: loop {
                    if !(prsvec_.prso <= i__1) {
                        _currentBlock = 23;
                        break;
                    }
                    if !(objcts_.oadv[
                             (prsvec_.prso - 1i32) as (usize)
                         ] != play_.winner || savep == v && (objcts_.otval[
                                                                 (prsvec_.prso - 1i32) as (usize)
                                                             ] <= 0i32)) {
                        f = 0i32;
                        rspsub_(580i32,objcts_.odesc2[(prsvec_.prso - 1i32) as (usize)]);
                        f1 = drop_(1i32);
                        if saveh != play_.here {
                            _currentBlock = 14;
                            break;
                        }
                    }
                    prsvec_.prso = prsvec_.prso + 1;
                }
                if _currentBlock == 23 {
                } else {
                    return;
                }
            }
        } else {
            i__1 = objcts_.olnt;
            prsvec_.prso = 1i32;
            'loop3: loop {
                if !(prsvec_.prso <= i__1) {
                    _currentBlock = 23;
                    break;
                }
                if !(qhere_(prsvec_.prso,play_.here) == 0 || objcts_.oflag1[
                                                                 (prsvec_.prso - 1i32) as (usize)
                                                             ] & 32768i32 == 0i32 || objcts_.oflag2[
                                                                                         (prsvec_.prso - 1i32) as (usize)
                                                                                     ] & 1024i32 != 0i32 || savep == v && (objcts_.otval[
                                                                                                                               (prsvec_.prso - 1i32) as (usize)
                                                                                                                           ] <= 0i32)) {
                    if !(objcts_.oflag1[
                             (prsvec_.prso - 1i32) as (usize)
                         ] & 8192i32 == 0i32 && (objcts_.oflag2[
                                                     (prsvec_.prso - 1i32) as (usize)
                                                 ] & 32i32 == 0i32)) {
                        f = 0i32;
                        rspsub_(580i32,objcts_.odesc2[(prsvec_.prso - 1i32) as (usize)]);
                        f1 = take_(1i32);
                        if saveh != play_.here {
                            _currentBlock = 7;
                            break;
                        }
                    }
                }
                prsvec_.prso = prsvec_.prso + 1;
            }
            if _currentBlock == 23 {
            } else {
                return;
            }
        }
        i = 581i32;
        if savep == v {
            i = 582i32;
        }
    }
    if f != 0 {
        rspeak_(i);
    }
}
