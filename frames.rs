//! This module contains the frames for the party parrot animation and
//! helper methods for determining offset values in the driver.

/// From a given offset value, calculate the frame that it corresponds to and
/// the offset within the given frame.
///
/// The return value is `(frame_idx, offset_in_frame)`.
pub(crate) fn calc_frame_and_offset(offset: u64) -> (usize, u64) {
    let mut offset_mod_total = offset % total_size() as u64;
    let frame = FRAMES
        .iter()
        .take_while(|f| {
            let frame_len = f.len() as u64;
            if offset_mod_total >= frame_len {
                offset_mod_total -= frame_len;
                true
            } else {
                false
            }
        })
        .count();
    (frame, offset_mod_total)
}

/// Calculate the total byte size of all frames added together.
fn total_size() -> usize {
    FRAMES.iter().map(|f| f.len()).sum()
}

/// Frames in sequential order.
pub(crate) const FRAMES: [&str; 10] = [
    FRAME0, FRAME1, FRAME2, FRAME3, FRAME4, FRAME5, FRAME6, FRAME7, FRAME8, FRAME9,
];

const FRAME0: &str = concat!(
    "\x1B[2J",
    r#"

                         .cccc;;cc;';c.           
                      .,:dkdc:;;:c:,:d:.          
                     .loc'.,cc::c:::,..;:.        
                   .cl;....;dkdccc::,...c;        
                  .c:,';:'..ckc',;::;....;c.      
                .c:'.,dkkoc:ok:;llllc,,c,';:.     
               .;c,';okkkkkkkk:;lllll,:kd;.;:,.   
               co..:kkkkkkkkkk:;llllc':kkc..oNc   
             .cl;.,oxkkkkkkkkkc,:cll;,okkc'.cO;   
             ;k:..ckkkkkkkkkkkl..,;,.;xkko:',l'   
            .,...';dkkkkkkkkkkd;.....ckkkl'.cO;   
         .,,:,.;oo:ckkkkkkkkkkkdoc;;cdkkkc..cd,   
      .cclo;,ccdkkl;llccdkkkkkkkkkkkkkkkd,.c;     
     .lol:;;okkkkkxooc::coodkkkkkkkkkkkko'.oc     
   .c:'..lkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkd,.oc     
  .lo;,:cdkkkkkkkkkkkkkkkkkkkkkkkkkkkkkkd,.c;     
,dx:..;lllllllllllllllllllllllllllllllllc'...     
cNO;........................................
"#,
);

const FRAME1: &str = concat!(
    "\x1B[2J",
    r#"
                .ckx;'........':c.                
             .,:c:::::oxxocoo::::,',.             
            .odc'..:lkkoolllllo;..;d,             
            ;c..:o:..;:..',;'.......;.            
           ,c..:0Xx::o:.,cllc:,'::,.,c.           
           ;c;lkXKXXXXl.;lllll;lKXOo;':c.         
         ,dc.oXXXXXXXXl.,lllll;lXXXXx,c0:         
         ;Oc.oXXXXXXXXo.':ll:;'oXXXXO;,l'         
         'l;;kXXXXXXXXd'.'::'..dXXXXO;,l'         
         'l;:0XXXXXXXX0x:...,:o0XXXXx,:x,         
         'l;;kXXXXXXXXXKkol;oXXXXXXXO;oNc         
        ,c'..ckk0XXXXXXXXXX00XXXXXXX0:;o:.        
      .':;..:do::ooookXXXXXXXXXXXXXXXo..c;        
    .',',:co0XX0kkkxxOXXXXXXXXXXXXXXXOc..;l.      
  .:;'..oXXXXXXXXXXXXXXXXXXXXXXXXXXXXXko;';:.     
.ldc..:oOXKXXXXXXKXXKXXXXXXXXXXXXXXXXXXXo..oc     
:0o...:dxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxo,.:,     
cNo........................................;' 
"#,
);

const FRAME2: &str = concat!(
    "\x1B[2J",
    r#"
            .cc;.  ...  .;c.                      
         .,,cc:cc:lxxxl:ccc:;,.                   
        .lo;...lKKklllookl..cO;                   
      .cl;.,:'.okl;..''.;,..';:.                  
     .:o;;dkd,.ll..,cc::,..,'.;:,.                
     co..lKKKkokl.':lloo;''ol..;dl.               
   .,c;.,xKKKKKKo.':llll;.'oOxl,.cl,.             
   cNo..lKKKKKKKo'';llll;;okKKKl..oNc             
   cNo..lKKKKKKKko;':c:,'lKKKKKo'.oNc             
   cNo..lKKKKKKKKKl.....'dKKKKKxc,l0:             
   .c:'.lKKKKKKKKKk;....lKKKKKKo'.oNc             
     ,:.'oxOKKKKKKKOxxxxOKKKKKKxc,;ol:.           
     ;c..'':oookKKKKKKKKKKKKKKKKKk:.'clc.         
   ,xl'.,oxo;'';oxOKKKKKKKKKKKKKKKOxxl:::;,.      
  .dOc..lKKKkoooookKKKKKKKKKKKKKKKKKKKxl,;ol.     
  cx,';okKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKl..;lc.   
  co..:dddddddddddddddddddddddddddddddddl::',::.  
co........................................... 
"#,
);

const FRAME3: &str = concat!(
    "\x1B[2J",
    r#"
           .ccccccc.                              
      .,,,;cooolccoo;;,,.                         
     .dOx;..;lllll;..;xOd.                        
   .cdo;',loOXXXXXkll;';odc.                      
  ,ol:;c,':oko:cccccc,...ckl.                     
  ;c.;kXo..::..;c::'.......oc                     
,dc..oXX0kk0o.':lll;..cxxc.,ld,                   
kNo.'oXXXXXXo',:lll;..oXXOo;cOd.                  
KOc;oOXXXXXXo.':lol;..dXXXXl';xc                  
Ol,:k0XXXXXX0c.,clc'.:0XXXXx,.oc                  
KOc;dOXXXXXXXl..';'..lXXXXXo..oc                  
dNo..oXXXXXXXOx:..'lxOXXXXXk,.:; ..               
cNo..lXXXXXXXXXOolkXXXXXXXXXkl,..;:';.            
.,;'.,dkkkkk0XXXXXXXXXXXXXXXXXOxxl;,;,;l:.        
  ;c.;:''''':doOXXXXXXXXXXXXXXXXXXOdo;';clc.      
  ;c.lOdood:'''oXXXXXXXXXXXXXXXXXXXXXk,..;ol.     
';.:xxxxxocccoxxxxxxxxxxxxxxxxxxxxxxl::'.';;.
  ';........................................;l'   
"#,
);

const FRAME4: &str = concat!(
    "\x1B[2J",
    r#"
                                                  
        .;:;;,.,;;::,.                            
     .;':;........'co:.                           
   .clc;'':cllllc::,.':c.                         
  .lo;;o:coxdllllllc;''::,,.                      
.c:'.,cl,.'l:',,;;'......cO;                      
do;';oxoc;:l;;llllc'.';;'.,;.                     
c..ckkkkkkkd,;llllc'.:kkd;.':c.                   
'.,okkkkkkkkc;lllll,.:kkkdl,cO;                   
..;xkkkkkkkkc,ccll:,;okkkkk:,co,                  
..,dkkkkkkkkc..,;,'ckkkkkkkc;ll.                  
..'okkkkkkkko,....'okkkkkkkc,:c.                  
c..ckkkkkkkkkdl;,:okkkkkkkkd,.',';.               
d..':lxkkkkkkkkxxkkkkkkkkkkkdoc;,;'..'.,.         
o...'';llllldkkkkkkkkkkkkkkkkkkdll;..'cdo.        
o..,l;'''''';dkkkkkkkkkkkkkkkkkkkkdlc,..;lc.      
o..;lc;;;;;;,,;clllllllllllllllllllllc'..,:c.     
o..........................................;' 
"#,
);

const FRAME5: &str = concat!(
    "\x1B[2J",
    r#"
                                                  
           .,,,,,,,,,.                            
         .ckKxodooxOOdcc.                         
      .cclooc'....';;cool.                        
     .loc;;;;clllllc;;;;;:;,.                     
   .c:'.,okd;;cdo:::::cl,..oc                     
  .:o;';okkx;';;,';::;'....,:,.                   
  co..ckkkkkddkc,cclll;.,c:,:o:.                  
  co..ckkkkkkkk:,cllll;.:kkd,.':c.                
.,:;.,okkkkkkkk:,cclll;.ckkkdl;;o:.               
cNo..ckkkkkkkkko,.;loc,.ckkkkkc..oc               
,dd;.:kkkkkkkkkx;..;:,.'lkkkkko,.:,               
  ;:.ckkkkkkkkkkc.....;ldkkkkkk:.,'               
,dc..'okkkkkkkkkxoc;;cxkkkkkkkkc..,;,.            
kNo..':lllllldkkkkkkkkkkkkkkkkkdcc,.;l.           
KOc,c;''''''';lldkkkkkkkkkkkkkkkkkc..;lc.         
xx:':;;;;,.,,...,;;cllllllllllllllc;'.;od,        
cNo.....................................oc 
"#,
);

const FRAME6: &str = concat!(
    "\x1B[2J",
    r#"
                                                  
                                                  
                   .ccccccc.                      
               .ccckNKOOOOkdcc.                   
            .;;cc:ccccccc:,:c::,,.                
         .c;:;.,cccllxOOOxlllc,;ol.               
        .lkc,coxo:;oOOxooooooo;..:,               
      .cdc.,dOOOc..cOd,.',,;'....':l.             
      cNx'.lOOOOxlldOc..;lll;.....cO;             
     ,do;,:dOOOOOOOOOl'':lll;..:d:''c,            
     co..lOOOOOOOOOOOl'':lll;.'lOd,.cd.           
     co.'dOOOOOOOOOOOo,.;llc,.,dOOc..dc           
     co..lOOOOOOOOOOOOc.';:,..cOOOl..oc           
   .,:;.'::lxOOOOOOOOOo:'...,:oOOOc.'dc           
   ;Oc..cl'':lldOOOOOOOOdcclxOOOOx,.cd.           
  .:;';lxl''''':lldOOOOOOOOOOOOOOc..oc            
,dl,.'cooc:::,....,::coooooooooooc'.c:            
cNo.................................oc 
"#,
);

const FRAME7: &str = concat!(
    "\x1B[2J",
    r#"
                                                  
                                                  
                                                  
                        .cccccccc.                
                  .,,,;;cc:cccccc:;;,.            
                .cdxo;..,::cccc::,..;l.           
               ,do:,,:c:coxxdllll:;,';:,.         
             .cl;.,oxxc'.,cc,.';;;'...oNc         
             ;Oc..cxxxc'.,c;..;lll;...cO;         
           .;;',:ldxxxdoldxc..;lll:'...'c,        
           ;c..cxxxxkxxkxxxc'.;lll:'','.cdc.      
         .c;.;odxxxxxxxxxxxd;.,cll;.,l:.'dNc      
        .:,''ccoxkxxkxxxxxxx:..,:;'.:xc..oNc      
      .lc,.'lc':dxxxkxxxxxxxol,...',lx:..dNc      
     .:,',coxoc;;ccccoxxxxxxxxo:::oxxo,.cdc.      
  .;':;.'oxxxxxc''''';cccoxxxxxxxxxxxc..oc        
,do:'..,:llllll:;;;;;;,..,;:lllllllll;..oc        
cNo.....................................oc 
"#,
);

const FRAME8: &str = concat!(
    "\x1B[2J",
    r#"

                                                  
                                                  
                              .ccccc.             
                         .cc;'coooxkl;.           
                     .:c:::c:,,,,,;c;;,.'.        
                   .clc,',:,..:xxocc;'..c;        
                  .c:,';:ox:..:c,,,,,,...cd,      
                .c:'.,oxxxxl::l:.,loll;..;ol.     
                ;Oc..:xxxxxxxxx:.,llll,....oc     
             .,;,',:loxxxxxxxxx:.,llll;.,,.'ld,   
            .lo;..:xxxxxxxxxxxx:.'cllc,.:l:'cO;   
           .:;...'cxxxxxxxxxxxxoc;,::,..cdl;;l'   
         .cl;':,'';oxxxxxxdxxxxxx:....,cooc,cO;   
     .,,,::;,lxoc:,,:lxxxxxxxxxxxo:,,;lxxl;'oNc   
   .cdxo;':lxxxxxxc'';cccccoxxxxxxxxxxxxo,.;lc.   
  .loc'.'lxxxxxxxxocc;''''';ccoxxxxxxxxx:..oc     
olc,..',:cccccccccccc:;;;;;;;;:ccccccccc,.'c,     
Ol;......................................;l' 
"#,
);

const FRAME9: &str = concat!(
    "\x1B[2J",
    r#"
                                                  
                              ,ddoodd,            
                         .cc' ,ooccoo,'cc.        
                      .ccldo;...',,...;oxdc.      
                   .,,:cc;.,'..;lol;;,'..lkl.     
                  .dOc';:ccl;..;dl,.''.....oc     
                .,lc',cdddddlccld;.,;c::'..,cc:.  
                cNo..:ddddddddddd;':clll;,c,';xc  
               .lo;,clddddddddddd;':clll;:kc..;'  
             .,c;..:ddddddddddddd:';clll,;ll,..   
             ;Oc..';:ldddddddddddl,.,c:;';dd;..   
           .''',:c:,'cdddddddddddo:,''..'cdd;..   
         .cdc';lddd:';lddddddddddddd;.';lddl,..   
      .,;::;,cdddddol;;lllllodddddddlcldddd:.'l;  
     .dOc..,lddddddddlcc:;'';cclddddddddddd;;ll.  
   .coc,;::ldddddddddddddlcccc:ldddddddddl:,cO;   
,xl::,..,cccccccccccccccccccccccccccccccc:;':xx,  
cNd.........................................;lOc 
"#,
);
