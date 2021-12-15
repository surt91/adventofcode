use super::AdventError;

pub fn parse(input: &str) -> Result<String, AdventError> {
    if input.trim() == A.trim() {
        Ok("A".to_string())
    } else if input.trim() == B.trim() {
        Ok("B".to_string())
    } else if input.trim() == C.trim() {
        Ok("C".to_string())
    } else if input.trim() == E.trim() {
        Ok("E".to_string())
    } else if input.trim() == F.trim() {
        Ok("F".to_string())
    } else if input.trim() == G.trim() {
        Ok("G".to_string())
    } else if input.trim() == H.trim() {
        Ok("H".to_string())
    } else if input.trim() == J.trim() {
        Ok("J".to_string())
    } else if input.trim() == K.trim() {
        Ok("K".to_string())
    } else if input.trim() == L.trim() {
        Ok("L".to_string())
    } else if input.trim() == O.trim() {
        Ok("O".to_string())
    } else if input.trim() == R.trim() {
        Ok("R".to_string())
    } else {
        Err(AdventError::IncompleteProgram { missing: input.trim().to_string() })
    }
}

const A: &str =
"
 #####
#  #
#  #
 #####
";
const B: &str =
"
######
# #  #
# #  #
 # ##
";
const C: &str =
"
 ####
#    #
#    #
 #  #
";
const E: &str =
"
######
# #  #
# #  #
#    #
";
const F: &str =
"
######
# #
# #
#
";
const G: &str =
"
 ####
#    #
#  # #
 # ###
";
const H: &str =
r"
######
  #
  #
######
";
const J: &str =
"
    #
     #
#    #
#####
";
const K: &str =
"
######
  #
 # ##
#    #
";
const L: &str =
"
######
     #
     #
     #
";
const O: &str =
"
#####
#   #
#   #
#   #
#####
";
const R: &str =
"
######
#  #
#  ##
 ##  #
";
