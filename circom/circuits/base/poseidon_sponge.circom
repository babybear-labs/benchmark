pragma circom 2.0.0;
include "../../node_modules/circomlib/circuits/poseidon.circom";

template SpongeHash(inputs_length){
    signal input in[inputs_length];
    signal output hash;

    var k = 0;
    var hash_blocks = (inputs_length\15) + 1;
    var block_index = 0;

    component poseidon_hash[hash_blocks];
    signal inside_hash[hash_blocks];
    signal frame[hash_blocks][16];

    for (var i=0; i<inputs_length; i++) {
        frame[block_index][k] <== in[i];

        if (k == 15){

            //*  START POSEIDON HASH
            poseidon_hash[block_index] = Poseidon(16);
            for (var j=0; j<16; j++) 
                poseidon_hash[block_index].inputs[j] <== frame[block_index][j];
            inside_hash[block_index] <== poseidon_hash[block_index].out; 
            block_index++;
            //*/ END POSEIDON HASH
            
            frame[block_index][0] <== inside_hash[block_index - 1];
            k = 1;
        } else 
            k++;
        
    }
    
    if (inputs_length%16 != 0){

        //*  START POSEIDON HASH
         poseidon_hash[block_index] = Poseidon(16);
         for (var j=0; j<16; j++) 
             poseidon_hash[block_index].inputs[j] <== j<k ? frame[block_index][j] : 0;
         hash <== poseidon_hash[block_index].out;
        //*/ END POSEIDON HASH
    
     }else
        hash <== frame[block_index][0];
    
   
    
}

//MAIN component main = SpongeHash(NUM);