#![allow(unused)]

use search::naive;
use search::rabin_karp;

pub const WORDS_15000_RAW: &'static str = include_str!("../data/words-15000");

fn main() {
    // let pattern = "unprepossessing narcissist amazingly microelectronics sassafrases barnacled shepherding capitalists horsetail unintentional grandchildren predestines undifferentiated possesses attenuate passivization reconcilable carpetbag shortsightedly combative photoengrave flowerpots fallibility graveyard denominated checkrooms hollowest extemporize mathematics abundantly limescale supporting fascicles translucency diplomatic carthorse gridlocked emasculating unassailable thingamabob counterclaimed contribution overzealous fatherlands bracketed animosity virtualization enriching repudiated redactors commandeered consideration mortuaries chamberlains cuckolded perturbing flabbiness crankshaft analogizes desensitizing lasciviousness placeholder shareholder architraves redelivering monopolizes mythology cutthroats philanthropically premiered imprecisely unsealing alphabetizations crossbowman rectifier exquisitely overwhelms whatshisname contradistinction inadequacies helicoptered tealights capability understudies grumpiest jeeringly Zarathustra jellyfishes steaminess Menkalinan netiquettes purposelessly impounding pinpoints southwards dysphagia wholesalers existence microwaving dishonors ineligibles lavaliere unaesthetic inflexibly acknowledging unforgivable barracuda constrictive marveling insularity preconceptions fiddlesticks redounded strikebreaker theologically abidingly falsifying collocate predominately transposition Turkmenistan schnitzels Chittagong corroding colloquially underacted motorcycle mouthpiece haystacks margaritas antipodals givebacks phenomenon ultrasound salesperson stomachache bombarding sandpapering";
    let pattern = "housewarming pentathlon spectacularly";
    let ind = rabin_karp::search(WORDS_15000_RAW, [pattern].into());
    // let ind = naive::search(WORDS_15000_RAW, pattern);

    if let Some(ind) = ind {
        println!("found pattern with len={} bytes at {} byte", pattern.len(), ind);
    } else {
        println!("didn't found pattern '{}'", pattern);
    }
}
