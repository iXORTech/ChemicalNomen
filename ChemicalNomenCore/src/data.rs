use serde::{Deserialize, Serialize};
use serde_json::Result;

const MAX_ELEMENT_NUMBER: i64 = 119;

const AVAILABLE_ELEMENTS: [&str; MAX_ELEMENT_NUMBER as usize] = [
    "hydrogen", "helium", "lithium", "beryllium", "boron", "carbon", "nitrogen", "oxygen",
    "fluorine", "neon", "sodium", "magnesium", "aluminium", "silicon", "phosphorus", "sulfur",
    "chlorine", "argon", "potassium", "calcium", "scandium", "titanium", "vanadium", "chromium",
    "manganese", "iron", "cobalt", "nickel", "copper", "zinc", "gallium", "germanium", "arsenic",
    "selenium", "bromine", "krypton", "rubidium", "strontium", "yttrium", "zirconium", "niobium",
    "molybdenum", "technetium", "ruthenium", "rhodium", "palladium", "silver", "cadmium", "indium",
    "tin", "antimony", "tellurium", "iodine", "xenon", "cesium", "barium", "lanthanum", "cerium",
    "praseodymium", "neodymium", "promethium", "samarium", "europium", "gadolinium", "terbium",
    "dysprosium", "holmium", "erbium", "thulium", "ytterbium", "lutetium", "hafnium", "tantalum",
    "tungsten", "rhenium", "osmium", "iridium", "platinum", "gold", "mercury", "thallium", "lead",
    "bismuth", "polonium", "astatine", "radon", "francium", "radium", "actinium", "thorium",
    "protactinium", "uranium", "neptunium", "plutonium", "americium", "curium", "berkelium",
    "californium", "einsteinium", "fermium", "mendelevium", "nobelium", "lawrencium",
    "rutherfordium", "dubnium", "seaborgium", "bohrium", "hassium", "meitnerium", "darmstadtium",
    "roentgenium", "copernicium", "nihonium", "flerovium", "moscovium", "livermorium", "tennessine",
    "oganesson", "ununennium"
];

const AVAILABLE_ELEMENTS_SYMBOLS: [&str; MAX_ELEMENT_NUMBER as usize] = [
    "H", "He", "Li", "Be", "B", "C", "N", "O", "F", "Ne", "Na", "Mg", "Al", "Si", "P", "S", "Cl",
    "Ar", "K", "Ca", "Sc", "Ti", "V", "Cr", "Mn", "Fe", "Co", "Ni", "Cu", "Zn", "Ga", "Ge", "As",
    "Se", "Br", "Kr", "Rb", "Sr", "Y", "Zr", "Nb", "Mo", "Tc", "Ru", "Rh", "Pd", "Ag", "Cd", "In",
    "Sn", "Sb", "Te", "I", "Xe", "Cs", "Ba", "La", "Ce", "Pr", "Nd", "Pm", "Sm", "Eu", "Gd", "Tb",
    "Dy", "Ho", "Er", "Tm", "Yb", "Lu", "Hf", "Ta", "W", "Re", "Os", "Ir", "Pt", "Au", "Hg", "Tl",
    "Pb", "Bi", "Po", "At", "Rn", "Fr", "Ra", "Ac", "Th", "Pa", "U", "Np", "Pu", "Am", "Cm", "Bk",
    "Cf", "Es", "Fm", "Md", "No", "Lr", "Rf", "Db", "Sg", "Bh", "Hs", "Mt", "Ds", "Rg", "Cn", "Nh",
    "Fl", "Mc", "Lv", "Ts", "Og", "Uue"
];

#[derive(Serialize, Deserialize)]
struct Element {
    name: String,
    root: String,
    symbol: String,
    appearance: String,
    atomic_mass: f64,
    boil: f64,
    category: String,
    density: f64,
    discovered_by: String,
    melt: f64,
    molar_heat: f64,
    named_by: String,
    number: i64,
    period: i64,
    group: i64,
    phase: String,
    summary: String,
    xpos: i64,
    ypos: i64,
    wxpos: i64,
    wypos: i64,
    shells: Vec<i64>,
    electron_configuration: String,
    electron_configuration_semantic: String,
    electron_affinity: f64,
    electronegativity_pauling: f64,
    ionization_energies: Vec<f64>,
    oxidation_states: Vec<i64>,
    cpk_hex_color: String,
    block: String,
}
