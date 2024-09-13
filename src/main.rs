use std::io::stdin;

mod main1;
mod main2;
mod main3;
mod main4;
mod main5;
mod main6;
mod main7;
mod main8;
mod main9;
mod main10;
mod main11;
mod main12;
mod main13;
mod main14;
mod main15;
mod main16;
mod main17;
mod main18;
mod main19;
mod main20;
mod main21;
mod main22;
mod main23;
mod main24;
mod main25;
mod main26;
mod main27;
mod main28;
mod main29;
mod main30;
mod main31;
mod main32;
mod main33;
mod main34;
mod main35;
mod main36;
mod main37;
mod main38;
mod main39;
mod main40;
mod main41;
mod main42;
mod main43;
mod main44;
mod main45;
mod main46;
mod main47;
mod main48;
mod main49;
mod main50;
mod main51;
mod main52;
mod main53;
mod main54;
mod main55;
mod main56;
mod main57;
mod main58;
mod main59;
mod main60;
mod main61;
mod main62;
mod main63;
mod main64;
mod main65;

fn main() {
    println!("Enter a number to run the file:");
    let mut input_line = String::new();
    if let Err(err) = stdin().read_line(&mut input_line) {
        println!("Error occurred while reading line. {err}")
    }
    match input_line.trim().parse() {
        Err(err) => {
            println!("Error while parsing the input. {err}")
        }
        Ok(num) => {
            match num {
                1 => { main1::main1() }
                2 => { main2::main2() }
                3 => { main3::main3() }
                4 => { main4::main4() }
                5 => { main5::main5() }
                6 => { main6::main6() }
                7 => { main7::main7() }
                8 => { main8::main8() }
                9 => { main9::main9() }
                10 => { main10::main10() }
                11 => { main11::main11() }
                12 => { main12::main12() }
                13 => { main13::main13() }
                14 => { main14::main14() }
                15 => { main15::main15() }
                16 => { main16::main16() }
                17 => { main17::main17() }
                18 => { main18::main18() }
                19 => { main19::main19() }
                20 => { main20::main20() }
                21 => { main21::main21() }
                22 => { main22::main22() }
                23 => { main23::main23() }
                24 => { main24::main24() }
                25 => { main25::main25() }
                26 => { main26::main26() }
                27 => { main27::main27() }
                28 => { main28::main28() }
                29 => { main29::main29() }
                30 => { main30::main30() }
                31 => { main31::main31() }
                32 => { main32::main32() }
                33 => { main33::main33() }
                34 => { main34::main34() }
                35 => { main35::main35() }
                36 => { main36::main36() }
                37 => { main37::main37() }
                38 => { main38::main38() }
                39 => { main39::main39() }
                40 => { main40::main40() }
                41 => { main41::main41() }
                42 => { main42::main42() }
                43 => { main43::main43() }
                44 => { main44::main44() }
                45 => { main45::main45() }
                46 => { main46::main46() }
                47 => { main47::main47() }
                48 => { main48::main48() }
                49 => { main49::main49() }
                50 => { main50::main50() }
                51 => { main51::main51() }
                52 => { main52::main52() }
                53 => { main53::main53() }
                54 => { main54::main54() }
                55 => { main55::main55() }
                56 => { main56::main56() }
                57 => { main57::main57() }
                58 => { main58::main58() }
                59 => { main59::main59() }
                60 => { main60::main60() }
                61 => { main61::main61() }
                62 => { main62::main62() }
                63 => { main63::main63() }
                64 => { main64::main64() }
                65 => { main65::main65()}
                _ => {
                    println!("Error, no such file.")
                }
            }
        }
    }
}
