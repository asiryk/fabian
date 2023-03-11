#![allow(unused)]

use search::naive;
use search::rabin_karp;
use memchr::memmem;

pub const WORDS_15000_RAW: &'static str = include_str!("../data/words-15000");

fn main() {
    // env_logger::builder().filter_level(log::LevelFilter::Trace).init();

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

/// Benchmark 1: fabian-bench -w 2 \
///   Time (mean ± σ):       0.8 ms ±   0.3 ms    [User: 0.4 ms, System: 0.3 ms] \
///   Range (min … max):     0.5 ms …   2.9 ms    583 runs
fn memchr_huge_haystack_large_needle() -> Option<usize> {
    let haystack = WORDS_15000_RAW.as_bytes();
    let needle = "unprepossessing narcissist amazingly microelectronics sassafrases barnacled shepherding capitalists horsetail unintentional grandchildren predestines undifferentiated possesses attenuate passivization reconcilable carpetbag shortsightedly combative photoengrave flowerpots fallibility graveyard denominated checkrooms hollowest extemporize mathematics abundantly limescale supporting fascicles translucency diplomatic carthorse gridlocked emasculating unassailable thingamabob counterclaimed contribution overzealous fatherlands bracketed animosity virtualization enriching repudiated redactors commandeered consideration mortuaries chamberlains cuckolded perturbing flabbiness crankshaft analogizes desensitizing lasciviousness placeholder shareholder architraves redelivering monopolizes mythology cutthroats philanthropically premiered imprecisely unsealing alphabetizations crossbowman rectifier exquisitely overwhelms whatshisname contradistinction inadequacies helicoptered tealights capability understudies grumpiest jeeringly Zarathustra jellyfishes steaminess Menkalinan netiquettes purposelessly impounding pinpoints southwards dysphagia wholesalers existence microwaving dishonors ineligibles lavaliere unaesthetic inflexibly acknowledging unforgivable barracuda constrictive marveling insularity preconceptions fiddlesticks redounded strikebreaker theologically abidingly falsifying collocate predominately transposition Turkmenistan schnitzels Chittagong corroding colloquially underacted motorcycle mouthpiece haystacks margaritas antipodals givebacks phenomenon ultrasound salesperson stomachache bombarding sandpapering".as_bytes();

    memmem::find(haystack, needle)
}

/// Benchmark 1: fabian-bench -w 2 \
///   Time (mean ± σ):       0.9 ms ±   0.3 ms    [User: 0.5 ms, System: 0.3 ms] \
///   Range (min … max):     0.6 ms …   3.7 ms    510 runs
fn naive_huge_haystack_large_needle() -> Option<usize> {
    let haystack = WORDS_15000_RAW.as_bytes();
    let needle = "unprepossessing narcissist amazingly microelectronics sassafrases barnacled shepherding capitalists horsetail unintentional grandchildren predestines undifferentiated possesses attenuate passivization reconcilable carpetbag shortsightedly combative photoengrave flowerpots fallibility graveyard denominated checkrooms hollowest extemporize mathematics abundantly limescale supporting fascicles translucency diplomatic carthorse gridlocked emasculating unassailable thingamabob counterclaimed contribution overzealous fatherlands bracketed animosity virtualization enriching repudiated redactors commandeered consideration mortuaries chamberlains cuckolded perturbing flabbiness crankshaft analogizes desensitizing lasciviousness placeholder shareholder architraves redelivering monopolizes mythology cutthroats philanthropically premiered imprecisely unsealing alphabetizations crossbowman rectifier exquisitely overwhelms whatshisname contradistinction inadequacies helicoptered tealights capability understudies grumpiest jeeringly Zarathustra jellyfishes steaminess Menkalinan netiquettes purposelessly impounding pinpoints southwards dysphagia wholesalers existence microwaving dishonors ineligibles lavaliere unaesthetic inflexibly acknowledging unforgivable barracuda constrictive marveling insularity preconceptions fiddlesticks redounded strikebreaker theologically abidingly falsifying collocate predominately transposition Turkmenistan schnitzels Chittagong corroding colloquially underacted motorcycle mouthpiece haystacks margaritas antipodals givebacks phenomenon ultrasound salesperson stomachache bombarding sandpapering".as_bytes();

    naive::search(haystack, needle)
}

/// Benchmark 1: fabian-bench -w 2 \
///   Time (mean ± σ):     959.7 ms ±   0.7 ms    [User: 953.7 ms, System: 4.6 ms] \
///   Range (min … max):   958.7 ms … 961.0 ms    10 runs
fn rabin_karp_huge_haystack_large_needle() -> Option<usize> {
    let haystack = WORDS_15000_RAW.as_bytes();
    let needle = "unprepossessing narcissist amazingly microelectronics sassafrases barnacled shepherding capitalists horsetail unintentional grandchildren predestines undifferentiated possesses attenuate passivization reconcilable carpetbag shortsightedly combative photoengrave flowerpots fallibility graveyard denominated checkrooms hollowest extemporize mathematics abundantly limescale supporting fascicles translucency diplomatic carthorse gridlocked emasculating unassailable thingamabob counterclaimed contribution overzealous fatherlands bracketed animosity virtualization enriching repudiated redactors commandeered consideration mortuaries chamberlains cuckolded perturbing flabbiness crankshaft analogizes desensitizing lasciviousness placeholder shareholder architraves redelivering monopolizes mythology cutthroats philanthropically premiered imprecisely unsealing alphabetizations crossbowman rectifier exquisitely overwhelms whatshisname contradistinction inadequacies helicoptered tealights capability understudies grumpiest jeeringly Zarathustra jellyfishes steaminess Menkalinan netiquettes purposelessly impounding pinpoints southwards dysphagia wholesalers existence microwaving dishonors ineligibles lavaliere unaesthetic inflexibly acknowledging unforgivable barracuda constrictive marveling insularity preconceptions fiddlesticks redounded strikebreaker theologically abidingly falsifying collocate predominately transposition Turkmenistan schnitzels Chittagong corroding colloquially underacted motorcycle mouthpiece haystacks margaritas antipodals givebacks phenomenon ultrasound salesperson stomachache bombarding sandpapering".as_bytes();

    rabin_karp::search(haystack, needle)
}

/// Benchmark 1: fabian-bench -w 20 \
///   Time (mean ± σ):       0.7 ms ±   0.3 ms    [User: 0.3 ms, System: 0.2 ms] \
///   Range (min … max):     0.3 ms …   3.1 ms    792 runs
fn naive_63_haystack() -> Option<usize> {
    let haystack = "limescale supporting fascicles translucency diplomatic carthorse".as_bytes();
    let needle = "carthorse".as_bytes();

    naive::search(haystack, needle)
}

/// Benchmark 1: fabian-bench -w 20 \
///   Time (mean ± σ):       0.7 ms ±   0.2 ms    [User: 0.3 ms, System: 0.3 ms] \
///   Range (min … max):     0.4 ms …   2.2 ms    857 runs
fn rabin_karp_63_haystack() -> Option<usize> {
    let haystack = "limescale supporting fascicles translucency diplomatic carthorse".as_bytes();
    let needle = "carthorse".as_bytes();

    rabin_karp::search(haystack, needle)
}

/// Benchmark 1: fabian-bench -w 20 \
///   Time (mean ± σ):       0.7 ms ±   0.2 ms    [User: 0.3 ms, System: 0.2 ms] \
///   Range (min … max):     0.3 ms …   1.7 ms    809 runs
fn rabin_karp_63_haystack_memchr() -> Option<usize> {
    let haystack = "limescale supporting fascicles translucency diplomatic carthorse".as_bytes();
    let needle = "carthorse".as_bytes();

    memmem::find(haystack, needle)
}
