

pub fn first_subword(mut s: String) -> String {
    if let  Some(pos) = s.char_indices().skip(1).find( |(_, ch)|   ch.is_uppercase() || *ch == '_'  ).map(|(index, _)| index) {  
            s.truncate(pos);    
    }
    s
   
   
}
