//继承，抽象到具体
//继承关系-path：：调用

    pub mod instrument{
        //pub:向上兼容
        //With this change, if we’re allowed to access sound , we can access instrument .
         pub mod woodwind {
             pub fn clarinet() {
             // Function body code goes here
                 super::breath();
             }
         }
         fn breath(){
             //woodwind::clarinet();
         }
     }
     mod voice{
 
     }
     
 