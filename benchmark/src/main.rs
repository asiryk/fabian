#![allow(unused)]

use search::naive;
use search::rabin_karp;
use memchr::memmem;

pub const WORDS_15000_RAW: &'static str = include_str!("../data/words-15000");

fn main() {
    env_logger::builder().filter_level(log::LevelFilter::Trace).init();

    // haystack = 63
    // let ind = rabin_karp_63_haystack_memchr();
    // let ind = rabin_karp_63_haystack();
    // let ind = naive_63_haystack();

    // haystack = full
    // let ind = naive_huge_haystack_large_needle();
    let ind = rabin_karp_huge_haystack_large_needle();
    // let ind = memchr_huge_haystack_large_needle();

    if let Some(ind) = ind {
        println!("found pattern at {} byte", ind);
    } else {
        println!("didn't found pattern");
    }
}

fn memchr_huge_haystack_large_needle() -> Option<usize> {
    let haystack = WORDS_15000_RAW.as_bytes();
    let needle = "unprepossessing narcissist amazingly microelectronics sassafrases barnacled shepherding capitalists horsetail unintentional grandchildren predestines undifferentiated possesses attenuate passivization reconcilable carpetbag shortsightedly combative photoengrave flowerpots fallibility graveyard denominated checkrooms hollowest extemporize mathematics abundantly limescale supporting fascicles translucency diplomatic carthorse gridlocked emasculating unassailable thingamabob counterclaimed contribution overzealous fatherlands bracketed animosity virtualization enriching repudiated redactors commandeered consideration mortuaries chamberlains cuckolded perturbing flabbiness crankshaft analogizes desensitizing lasciviousness placeholder shareholder architraves redelivering monopolizes mythology cutthroats philanthropically premiered imprecisely unsealing alphabetizations crossbowman rectifier exquisitely overwhelms whatshisname contradistinction inadequacies helicoptered tealights capability understudies grumpiest jeeringly Zarathustra jellyfishes steaminess Menkalinan netiquettes purposelessly impounding pinpoints southwards dysphagia wholesalers existence microwaving dishonors ineligibles lavaliere unaesthetic inflexibly acknowledging unforgivable barracuda constrictive marveling insularity preconceptions fiddlesticks redounded strikebreaker theologically abidingly falsifying collocate predominately transposition Turkmenistan schnitzels Chittagong corroding colloquially underacted motorcycle mouthpiece haystacks margaritas antipodals givebacks phenomenon ultrasound salesperson stomachache bombarding sandpapering".as_bytes();

    memmem::find(haystack, needle)
}

fn naive_huge_haystack_large_needle() -> Option<usize> {
    let haystack = WORDS_15000_RAW.as_bytes();
    let needle = "unprepossessing narcissist amazingly microelectronics sassafrases barnacled shepherding capitalists horsetail unintentional grandchildren predestines undifferentiated possesses attenuate passivization reconcilable carpetbag shortsightedly combative photoengrave flowerpots fallibility graveyard denominated checkrooms hollowest extemporize mathematics abundantly limescale supporting fascicles translucency diplomatic carthorse gridlocked emasculating unassailable thingamabob counterclaimed contribution overzealous fatherlands bracketed animosity virtualization enriching repudiated redactors commandeered consideration mortuaries chamberlains cuckolded perturbing flabbiness crankshaft analogizes desensitizing lasciviousness placeholder shareholder architraves redelivering monopolizes mythology cutthroats philanthropically premiered imprecisely unsealing alphabetizations crossbowman rectifier exquisitely overwhelms whatshisname contradistinction inadequacies helicoptered tealights capability understudies grumpiest jeeringly Zarathustra jellyfishes steaminess Menkalinan netiquettes purposelessly impounding pinpoints southwards dysphagia wholesalers existence microwaving dishonors ineligibles lavaliere unaesthetic inflexibly acknowledging unforgivable barracuda constrictive marveling insularity preconceptions fiddlesticks redounded strikebreaker theologically abidingly falsifying collocate predominately transposition Turkmenistan schnitzels Chittagong corroding colloquially underacted motorcycle mouthpiece haystacks margaritas antipodals givebacks phenomenon ultrasound salesperson stomachache bombarding sandpapering".as_bytes();

    naive::search(haystack, needle)
}

fn rabin_karp_huge_haystack_large_needle() -> Option<usize> {
    let haystack = WORDS_15000_RAW.as_bytes();
    let needle = "unprepossessing narcissist amazingly microelectronics sassafrases barnacled shepherding capitalists horsetail unintentional grandchildren predestines undifferentiated possesses attenuate passivization reconcilable carpetbag shortsightedly combative photoengrave flowerpots fallibility graveyard denominated checkrooms hollowest extemporize mathematics abundantly limescale supporting fascicles translucency diplomatic carthorse gridlocked emasculating unassailable thingamabob counterclaimed contribution overzealous fatherlands bracketed animosity virtualization enriching repudiated redactors commandeered consideration mortuaries chamberlains cuckolded perturbing flabbiness crankshaft analogizes desensitizing lasciviousness placeholder shareholder architraves redelivering monopolizes mythology cutthroats philanthropically premiered imprecisely unsealing alphabetizations crossbowman rectifier exquisitely overwhelms whatshisname contradistinction inadequacies helicoptered tealights capability understudies grumpiest jeeringly Zarathustra jellyfishes steaminess Menkalinan netiquettes purposelessly impounding pinpoints southwards dysphagia wholesalers existence microwaving dishonors ineligibles lavaliere unaesthetic inflexibly acknowledging unforgivable barracuda constrictive marveling insularity preconceptions fiddlesticks redounded strikebreaker theologically abidingly falsifying collocate predominately transposition Turkmenistan schnitzels Chittagong corroding colloquially underacted motorcycle mouthpiece haystacks margaritas antipodals givebacks phenomenon ultrasound salesperson stomachache bombarding sandpapering".as_bytes();

    rabin_karp::search(haystack, needle)
}

fn naive_63_haystack() -> Option<usize> {
    let haystack = "limescale supporting fascicles translucency diplomatic carthorse".as_bytes();
    let needle = "carthorse".as_bytes();

    naive::search(haystack, needle)
}

fn rabin_karp_63_haystack() -> Option<usize> {
    let haystack = "limescale supporting fascicles translucency diplomatic carthorse".as_bytes();
    let needle = "carthorse".as_bytes();

    rabin_karp::search(haystack, needle)
}

fn rabin_karp_63_haystack_memchr() -> Option<usize> {
    let haystack = "limescale supporting fascicles translucency diplomatic carthorse".as_bytes();
    let needle = "carthorse".as_bytes();

    memmem::find(haystack, needle)
}
