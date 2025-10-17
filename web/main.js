// main.js (type=module)
// -- service worker registration unchanged
if ("serviceWorker" in navigator) {
    navigator.serviceWorker.register(new URL("./sw.js", import.meta.url)).then(
        (registration) => {
            console.log("COOP/COEP Service Worker registered", registration.scope);
            if (registration.active && !navigator.serviceWorker.controller) {
                window.location.reload();
            }
        },
        (err) => {
            console.log("COOP/COEP Service Worker failed to register", err);
        }
    );
} else {
    console.warn("Cannot register a service worker");
}

const GH_OWNER = 'Fire-Aalt';
const GH_REPO = 'ib_pcode_compiler';

const RESPONSE_BYTES = 8192;
const sab = new SharedArrayBuffer(Int32Array.BYTES_PER_ELEMENT + RESPONSE_BYTES);
const control = new Int32Array(sab, 0, 1);
const respBuf = new Uint8Array(sab, Int32Array.BYTES_PER_ELEMENT, RESPONSE_BYTES);
control[0] = 0;
const worker = new Worker(new URL('./worker.js', import.meta.url), { type: 'module' });

/* DOM handles */
const terminal = document.getElementById('terminal');
const editor = document.getElementById('editor');
const gutter = document.getElementById('gutter');
const sampleSelect = document.getElementById('sampleSelect');

const modal = document.getElementById('modal');
const modalPrompt = document.getElementById('modalPrompt');
const modalInput = document.getElementById('modalInput');
const modalOk = document.getElementById('modalOk');

const themeToggle = document.getElementById('themeToggle');
const reportBtn = document.getElementById('reportBtn');
const githubBtn = document.getElementById('githubBtn');
const runBtn = document.getElementById('runBtn');
const saveBtn = document.getElementById('saveBtn');
const fileLabel = document.querySelector('.file-label');

let lastRequestId = null;
let currentRunWindow = null;

/* Tabs: update toolbar visibility on tab change */
function setActiveTab(tabName) {
    // tabName is 'editor'|'docs' etc.
    document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
    const btn = document.querySelector(`.tab[data-tab="${tabName}"]`);
    if (btn) btn.classList.add('active');

    document.querySelectorAll('.tab-panel').forEach(p => p.hidden = true);
    const panel = document.getElementById(tabName + 'Tab');
    if (panel) panel.hidden = false;

    // hide toolbar controls when docs tab active
    const hide = (tabName === 'docs');
    sampleSelect.style.display = hide ? 'none' : '';
    saveBtn.style.display = hide ? 'none' : '';
    fileLabel.style.display = hide ? 'none' : '';
    runBtn.style.display = hide ? 'none' : '';
}

// wire tab buttons
document.querySelectorAll('.tab').forEach(btn => {
    btn.addEventListener('click', () => {
        // report button may not have data-tab – handle separately
        const tab = btn.dataset.tab;
        if (tab) setActiveTab(tab);
    });
});

// initialize active tab on load
document.addEventListener('DOMContentLoaded', () => {
    setActiveTab('editor');
});

/* Theme toggle (follows system unless user chooses) */
const THEME_KEY = 'ibp-theme';
function getSystemPref() {
    return window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
}
function applyTheme(theme) {
    if (theme === 'dark') {
        document.body.setAttribute('data-theme', 'dark');
        themeToggle.textContent = '☀️';
        themeToggle.title = 'Switch to light theme';
    } else {
        document.body.removeAttribute('data-theme');
        themeToggle.textContent = '🌙';
        themeToggle.title = 'Switch to dark theme';
    }
}
const savedTheme = localStorage.getItem(THEME_KEY);
if (savedTheme === 'dark' || savedTheme === 'light') {
    applyTheme(savedTheme);
} else {
    applyTheme( getSystemPref() );
    // follow system if user hasn't chosen
    if (window.matchMedia) {
        const mq = window.matchMedia('(prefers-color-scheme: dark)');
        mq.addEventListener('change', (ev) => {
            if (!localStorage.getItem(THEME_KEY)) applyTheme(ev.matches ? 'dark' : 'light');
        })
    }
}
themeToggle.addEventListener('click', () => {
    const newTheme = document.body.hasAttribute('data-theme') ? 'light' : 'dark';
    applyTheme(newTheme);
    localStorage.setItem(THEME_KEY, newTheme);
});

/* Report button opens GitHub issues directly (prefills snapshot) */
reportBtn.addEventListener('click', () => {
    const title = encodeURIComponent('Bug report: [short description]');
    const body = encodeURIComponent(`**Describe the bug or feedback**\n\n**Editor snapshot:**\n\`\`\`\n${editor.value}\n\`\`\`\n\n*(Add steps to reproduce, browser, OS, WASM version, etc.)*`);
    const url = `https://github.com/${GH_OWNER}/${GH_REPO}/issues/new?title=${title}&body=${body}`;
    window.open(url, '_blank', 'noopener');
});

githubBtn.addEventListener('click', () => {
    const url = `https://github.com/${GH_OWNER}/${GH_REPO}`;
    window.open(url, '_blank', 'noopener');
});

/* ---- Samples: apply and reset to default ---- */
const samples = {
    welcome: `
output "Welcome"
loop COUNT from 1 to 5
  output COUNT
end loop
`,

    calculations: `
output "=== Simple Calculations ==="

output "Adding 1...10 = " , 1+2+3+4+5+6+7+8+9+10

output "10 Factorial = " , 1*2*3*4*5*6*7*8*9*10

output "Fractions = 1/2 + 1/4 + 1/5 = " , 1/2 + 1/4 + 1/5

output "Pythagoras = 3^2 + 4^2 = 5^2 = " , 3*3 + 4*4 , " and " , 5*5

output "Big Numbers = one trillion ^ 2 = " , 1000000000000 * 1000000000000

output "Easier big numbers = " , 2e12 * 3e12

output "10307 is not prime = " , 10307 / 11 , " * " , 11

output "15% of 12345 = " , 15/100*12345

output "Incorrect calculation = " , 1234567890 * 1234567890

output "Another error = " , 1/2 + 1/3 + 1/6

output "One more problem = " , 0.1+0.1+0.1+0.1+0.1+0.1+0.1+0.1

output "And another problem = " , 3.2 - 0.3
`,
    
    solve_equations: `
X = 4
Y = X*X - 9*X + 14
output "x = " , X , " .... y = " , Y
`,
    solving2: `
A = 10
B = 100
output "Sum = " , A + B
output "Product = " , A * B
`,
    ski_trip: `
CARS = 8
BUSSES = 8
HOTEL = 10
LODGE = 12
SEATS = CARS*4 + BUSSES*20
BEDS = HOTEL*4 + LODGE*12
COST = CARS*250 + BUSSES*1000 + HOTEL*300 + LODGE*800

output CARS , " cars and " , BUSSES , " busses = " , SEATS , " seats"
output HOTEL , " rooms and " , LODGE , " lodges = " , BEDS , " beds"
output "Total cost = " , COST
`,
    if_then: `
UNIT = input("Type a unit")

if  UNIT = "km"  then
    output "1 km = 1000 m = 0.6 miles"
end if

if  UNIT = "mi"  then
    output "1 mi = 5280 ft = 1.6 km"
end if

if  UNIT = "ft"  then
    output "1 ft = 12 in = 30.5 cm"
end if

if  UNIT = "liter"  then
    output "1 liter = 1000 ml = 1/3 gallon"
    output "Don't forget that IMPERIAL GALLONS"
    output "are different than US GALLONS"
end if
`,
    password_logic: `
NAME = input("Type your user name")
PASSWORD = input("Type your password")

if  NAME = "bozo"  AND  PASSWORD = "clown"  then
    output "Correct!"
end if

if  NAME = "einstein"  AND  PASSWORD = "e=mc2"  then
    output "Correct!"
end if

if  NAME = "guest"  OR  NAME = "trial"  then
    output "You will be logged in as a GUEST"
end if
`,
    discount_logic: `
QUANTITY = input("How many hamburgers do you want?")
PRICE = 0

if  QUANTITY >= 10  then
    PRICE = 2.59
else if  QUANTITY <= 9  AND  QUANTITY >= 5  then
    PRICE = 2.89
else if  QUANTITY < 5  then
    PRICE = 3.25
end if

output "That costs " , PRICE , " per burger"
output "Total cost = " , PRICE * QUANTITY , " for " , QUANTITY , " burgers"
`,
    mice_loops: `
loop A from 1 to 2
   output "Three blind mice"
end loop

loop B from 3 to 4
   output "See how they run"
end loop

output "They all ran up to the farmer's wife"
output "She cut off their tails with a carving knife"
output "Did you ever see such a sight in your life, as"

C = 5
loop while C < 20
   output "Three blind mice"
   C = C*2
end loop
`,
    money_decisions: `
EUROS = 50.00
POUNDS = 0.8*EUROS
DOLLARS = EUROS / 0.75
YEN = EUROS * 90

output EUROS , " EUR"
output YEN , " Yen"

if YEN > 1000 then
    output "That is a lot of Yen"
end if

output POUNDS , " BP"

if POUNDS < 100 then
    output "That is a small number of Pounds"
end if

output "$" , DOLLARS

if DOLLARS = 100 then
    output "BINGO"
end if
`,
    common_factors: `
A = 28
B = 42

output "Common factors of " , A , " and " , B

loop C from 1 to B
    if (A mod C = 0) AND (B mod C = 0) then
       output C
    end if
end loop
`,
    math_values: `
output "X , Y"

loop C from 0 to 10
   X = C / 2.0
   Y = 3*X*X - 7*X + 2
   output X , " , " , Y
end loop
`,
    collatz_sequence: `
NUM = 29

loop until NUM = 1
    output NUM

    if NUM mod 2 = 0 then
        NUM = NUM / 2
    else
        NUM = NUM * 3 + 1
    end if
end loop

output NUM
`,
    translate_strings: `
input ENGLISH
GERMAN = ""

if ENGLISH = "hello" then
    GERMAN = "guten Tag"
else if ENGLISH = "goodbye" then
    GERMAN = "auf Wiedersehen"
else if ENGLISH = "stop" then
    GERMAN = "halt"
else
    GERMAN = "???"
end if

output "English = " , ENGLISH
output "German = " , GERMAN
`,
    elapsed_minutes: `
input START_HOURS
input START_MINUTES

input END_HOURS
input END_MINUTES

if START_HOURS > 23 OR START_MINUTES > 59 then
    output "Start time is not valid"
else if END_HOURS > 23 OR END_MINUTES > 59 then
    output "Times are not valid"
else
    MINUTES = (END_HOURS - START_HOURS)*60 + (END_MINUTES-START_MINUTES)
    output "Elapsed time = " , MINUTES , " minutes"
end if
`,
    date_validation: `
input MONTH
input DAY
input YEAR

output MONTH , "/" , DAY , "/" , YEAR

FEBMAX = 0
if YEAR mod 4 = 0 then
   FEBMAX = 29
else
   FEBMAX = 28
end if

M = MONTH
D = DAY

if M < 1 OR M > 12 then
    output "Month is not valid"
else if D < 1 OR D > 31 then
    output "Day is not valid"
else if D = 31 AND (M = 4 OR M = 6 OR M = 9 OR M = 11) then
    output "That month does not have 31 days"
else if M = 2 AND D > FEBMAX then
    output "February only has " , FEBMAX , " days"
else
    output "Date is valid"
end if
`,
    add_up_number: `
MAX = 10

SUM = 0

loop COUNT from 0 to MAX
    output COUNT
    SUM = SUM + COUNT
end loop

output "Total = " , SUM
`,
    primes_array: `
NUMS = new Array()

loop N from 1 to 100
   NUMS[N] = 0
end loop

loop P from 2 to 50
   N = P * 2
   loop while N <= 100
      NUMS[N] = 1
      N = N + P
   end loop
end loop

output "These are the PRIME numbers under 100"

loop N from 2 to 100
   if NUMS[N] = 0 then
      output N
   end if
end loop
`,
    binary_search: `
ID = [1001,1002,1050,1100,1120,1180,1200,1400]
NAME = ["Apple","Cherry","Peach","Banana","Fig","Grape","Olive","Mango"]

output "Type the ID number that you wish to find"
input TARGET

LOW = 0
HIGH = 7
FOUND = -1

loop while FOUND = -1 AND LOW <= HIGH
    MID = div( LOW + HIGH , 2 )   // should be (LOW + HIGH) div 2
    // but (A div B) doesn't work correctly
    // so there is a special method below
    if ID[MID] = TARGET then
        FOUND = MID
    else if TARGET < ID[MID] then
        HIGH = MID - 1
    else
        LOW = MID + 1
    end if
end while

if FOUND >= 0 then
    output TARGET , ":" , NAME[FOUND]
else
    output TARGET , " was not found"
end if
`,
    selection_sort: `
NUMS = [15,30,85,25,40,90,50,65,20,60]

output "Before sorting"
printNums()

loop FIRST from 0 to 9
    LEAST = FIRST
    loop CURRENT from FIRST+1 to 9
        if NUMS[CURRENT] > NUMS[LEAST] then
           LEAST = CURRENT
        end if
    end loop
    TEMP = NUMS[LEAST]
    NUMS[LEAST] = NUMS[FIRST]
    NUMS[FIRST] = TEMP
end loop

output "After sorting"
printNums()

method printNums()
   loop C from 0 to 9
      output NUMS[C]
   end loop
   output "========"
end method
`,
    bubble_sort: `
NUMS = [15,30,85,25,40,90,50,65,20,60]

output "Before sorting"
loop C from 0 to 9
   output NUMS[C]
end loop

output "========"

loop PASS from 0 to 8
    loop CURRENT from 0 to 8
        if NUMS[CURRENT] < NUMS[CURRENT + 1] then
            TEMP = NUMS[CURRENT]
            NUMS[CURRENT] = NUMS[CURRENT+1]
            NUMS[CURRENT+1] = TEMP
        end if
    end loop
end loop

output "After sorting"

loop C from 0 to 9
   output NUMS[C]
end loop
`,
    reverse_array: `
NAMES = ["Robert","Boris","Brad","George","David"]

N = 5     // the number of elements in the array
K = 0     // this is the first index in the array

loop while K < N - 1
    TEMP = NAMES[K]
    NAMES[K] = NAMES[N - K - 1]
    NAMES[N - K - 1] = TEMP
    K = K + 1
end loop

loop C from 0 to N-1
    output NAMES[C]
end loop
`,
    frequency_distribution: `
DATA = [17,20,23,29,33,42,60,61,75,75,90,99]
FREQS = [0,0,0,0,0,0,0,0,0,0]

loop C from 0 to 11
    VALUE = DATA[C]
    loop F from 0 to 9
        if VALUE >= 10*F AND VALUE < 10*(F+1) then
            FREQS[F] = FREQS[F] + 1
        end if
    end loop
end loop

output "Data"

loop D from 0 to 11
    output DATA[D]
end loop

output "Range : Frequency"

loop F from 0 to 9
    output F*10 , " - " , (F+1)*10 , " : " , FREQS[F]
end loop
`,
    appointments_list: `
APPS = new Array()
NAME = ""

loop T from 0 to 2400
APPS[T] = ""
end loop

loop while NAME <> "quit"
    input NAME
    input TIME
    if TIME >= 0 AND TIME <= 2359 then
        APPS[TIME] = NAME
    end if

    loop T from 0 to 2400
        if APPS[T] <> "" then
            output T , " : " , APPS[T]
        end if
    end loop
    output "=================="
end loop
`,
    find_duplicates: `
TENNIS = ["Al","Bobby","Carla","Dave","Ellen"]
BBALL = ["Lou","Dave","Hellen","Alan","Al"]

output "The following appear in both lists"

loop T from 0 to 4
    loop B from 0 to 4
        if TENNIS[T] = BBALL[B] then
            output TENNIS[T]
        end if
    end loop
end loop
`,
    cities_array: `
CITIES = ["Athens","Berlin","Dallas","Denver","London","New York","Rome"]

COUNT = 0

loop C from 0 to 6
    if firstLetter( CITIES[C] ) = "D" then
        COUNT = COUNT + 1
        output CITIES[C]
    end if
end loop

output "That was " , COUNT , " D-cities"

method firstLetter(s)
    return s.substring(0,1)
end method
`,
    names_collection: `
NAMES = new Collection()

NAMES.addItem("Bob")
NAMES.addItem("Dave")
NAMES.addItem("Betty")
NAMES.addItem("Kim")
NAMES.addItem("Debbie")
NAMES.addItem("Lucy")

NAMES.resetNext()

output "These names start with D"

loop while NAMES.hasNext()
    NAME = NAMES.getNext()
    if firstLetter(NAME) = "D" then
      output NAME
    end if
end loop

method firstLetter(s)
   return s.substring(0,1)
end method
`,
    checkout_collection: `
NAMES = new Collection()
NAME = ""

loop while NAME <> "quit"
   input NAME
   if NAME <> "quit" then
       if NAMES.contains(NAME) then
           output NAME , " returned"
           NAMES.remove(NAME)
       else
           output NAME , " is leaving"
           NAMES.addItem(NAME)
       end if
   end if
end loop

output "The following students left and did not return"

NAMES.resetNext()

loop while NAMES.hasNext()
   output NAMES.getNext()
end loop
`,
    stack_reverse_list: `
NAMES = ["Alex","Bobby","Cho","Deke"]

STACK = new Stack()

loop COUNT from 0 to 3
   STACK.push(NAMES[COUNT])
end loop

loop while NOT(STACK.isEmpty())
   NAME = STACK.pop()
   output NAME
end loop
`,
    queues_merge: `
PEOPLE = ["Alex","Bobby","Cho","Deke","Ellen"]
DOGS = ["spot","woofie","bruiser"]

PQ = new Queue()          // Queue for People names
DQ = new Queue()          // Queue for dog names

/////////////////////////// copy people names
loop P from 0 to 4
   PQ.enqueue(PEOPLE[P])
end loop

//////////////////////////// copy dog names
loop D from 0 to 2
   DQ.enqueue(DOGS[D])
end loop

loop while NOT(PQ.isEmpty()) OR NOT(DQ.isEmpty())
   if NOT(PQ.isEmpty()) then
      output "Person = " , PQ.dequeue()
   else
      output "People list is empty"
   end if

   if NOT(DQ.isEmpty()) then
      output "Dog = " , DQ.dequeue()
   else
      output "Dog list is empty"
   end if
end loop
`,
    bank_classes: `
Class Account(name,amount)
    this.id = name
    this.balance = amount

    this.addInterest = function(percent)
    {
       this.balance = this.balance + this.balance*percent/100
    }

    this.addMoney = function(money)
    {
       this.balance = this.balance + money
    }

    this.show = function()
    {
       output this.id + " " + this.balance
    }
end Class

PAYMENTS = new Account("Abbey",100.0)

INTEREST = new Account("Pat",100.0)

loop YEARS from 0 to 10
    output "== Year : " + YEARS + " =="
    PAYMENTS.show()
    INTEREST.show()

    PAYMENTS.addMoney(100)
    INTEREST.addInterest(10)
end loop
`,
    magic_square: `
A = [
  [8,1,6] ,
  [3,5,7] ,
  [4,9,2]
]

OK = "correct"

loop R from 0 to 2
   output A[R][0] , A[R][1] , A[R][2]
end loop

loop R from 0 to 2
   SUM = 0
   loop C from 0 to 2
      SUM = SUM + A[R][C]
   end loop

   if SUM != 15 then
      output "Row " , R , " is wrong"
      OK = "wrong"
   end if
end loop

loop C from 0 to 2
   SUM = 0
   loop R from 0 to 2
      SUM = SUM + A[R][C]
   end loop

   if SUM != 15 then
      output "Column " , C , " is wrong"
      OK = "wrong"
   end if
end loop

SUM = 0
loop X from 0 to 2
   R = X
   C = X
   SUM = SUM + A[R][C]
end loop

if SUM != 15 then
   output "Main diag is wrong"
   OK = "wrong"
end if

SUM = 0
loop X from 0 to 2
   R = X
   C = 2-X
   SUM = SUM + A[R][C]
end loop

if SUM != 15 then
   output "Other diag is wrong"
   OK = "wrong"
end if

output "Entire square is " , OK
`,
};

function nicifyKey(key) {
    if (!key) return key;
    let s = key.replace(/[_-]+/g, ' ');
    s = s.replace(/([a-z0-9])([A-Z])/g, '$1 $2');
    s = s.replace(/([A-Z])([A-Z][a-z])/g, '$1 $2');
    s = s.replace(/\s+/g, ' ').trim();
    s = s.split(' ').map(w => w.charAt(0).toUpperCase() + w.slice(1).toLowerCase()).join(' ');
    return s;
}

for (const k of Object.keys(samples)) {
    const opt = document.createElement('option');
    opt.value = k;
    opt.textContent = nicifyKey(k);
    sampleSelect.appendChild(opt);
}

sampleSelect.addEventListener('change', () => {
    const v = sampleSelect.value;
    if (!v) return;
    editor.value = samples[v];
    updateGutter();

    // reset dropdown to the default label after a brief moment so user sees the option applied
    setTimeout(() => {
        sampleSelect.value = '';
    }, 150);
});

/* Gutter / editor logic (unchanged) */
function updateGutter() {
    const lines = Math.max(1, editor.value.split('\n').length);
    gutter.innerHTML = '';
    const cs = getComputedStyle(editor);
    let lineHeight = parseFloat(cs.lineHeight);
    if (!Number.isFinite(lineHeight) || lineHeight === 0) lineHeight = 20;
    for (let i = 1; i <= lines; i++) {
        const d = document.createElement('div');
        d.textContent = i.toString();
        d.style.height = lineHeight + 'px';
        d.style.lineHeight = lineHeight + 'px';
        gutter.appendChild(d);
    }
}
editor.addEventListener('input', () => updateGutter());

editor.addEventListener('keydown', (e) => {
    if (e.key === 'Tab') {
        e.preventDefault();
        const start = editor.selectionStart;
        const end = editor.selectionEnd;
        const value = editor.value;
        const tabChar = '\t';
        const selStartLine = value.slice(0, start).split('\n').length - 1;
        const selEndLine = value.slice(0, end).split('\n').length - 1;

        if (selStartLine !== selEndLine || (start !== end && value.slice(start, end).includes('\n'))) {
            const lines = value.split('\n');
            let lineStartIndices = [];
            let acc = 0;
            for (let i = 0; i < lines.length; i++) {
                lineStartIndices.push(acc);
                acc += lines[i].length + 1;
            }
            const isUnindent = e.shiftKey;
            for (let li = selStartLine; li <= selEndLine; li++) {
                if (!isUnindent) lines[li] = tabChar + lines[li];
                else {
                    if (lines[li].startsWith('\t')) lines[li] = lines[li].slice(1);
                    else if (lines[li].startsWith('    ')) lines[li] = lines[li].slice(4);
                }
            }
            const newValue = lines.join('\n');
            const newStart = lineStartIndices[selStartLine] + (isUnindent ? 0 : tabChar.length);
            const newEnd = lineStartIndices[selEndLine] + lines[selEndLine].length + 1;
            editor.value = newValue;
            editor.selectionStart = newStart;
            editor.selectionEnd = newEnd - 1;
            setTimeout(updateGutter, 0);
            return;
        }

        const before = value.slice(0, start);
        const after = value.slice(end);
        editor.value = before + tabChar + after;
        const caret = start + tabChar.length;
        editor.selectionStart = editor.selectionEnd = caret;
        setTimeout(updateGutter, 0);
        return;
    }
    setTimeout(updateGutter, 0);
});
updateGutter();

/* Save/Load/Run wiring (unchanged) */
saveBtn.addEventListener('click', () => {
    const blob = new Blob([editor.value], { type: 'text/plain' });
    const a = document.createElement('a');
    a.href = URL.createObjectURL(blob);
    a.download = 'program.pseudo';
    a.click();
    URL.revokeObjectURL(a.href);
});
document.getElementById('fileInput').addEventListener('change', (ev) => {
    const f = ev.target.files[0]; 
    if (!f) return;
    const r = new FileReader(); 
    r.onload = () => { 
        editor.value = r.result;
        updateGutter();
    }; 
    r.readAsText(f);
});
runBtn.addEventListener('click', () => {
    currentRunWindow = null;
    terminal.innerHTML = '';
    Atomics.store(control, 0, 0);
    Atomics.notify(control, 0, 1);
    const src = editor.value;
    worker.postMessage({ type: 'run', source: src, runId: Date.now() });
});

/* Worker messaging (unchanged) */
worker.onmessage = (ev) => {
    const msg = ev.data;
    if (msg.type === 'wasm-ready') {
        console.log('wasm-ready', msg);
    } else if (msg.type === 'request-input') {
        lastRequestId = msg.id;
        showModalPrompt(msg.prompt);
    } else if (msg.type === 'output') {
        appendOutput(msg.text);
    } else if (msg.type === 'error') {
        appendOutput('ERROR: ' + msg.message);
    } else {
        if (msg.text) appendOutput(msg.text);
    }
};
worker.postMessage({ type: 'init', controlSab: sab });

function writeResponseAndWake(text) {
    const enc = new TextEncoder();
    const encoded = enc.encode(text || '');
    const writeLen = Math.min(encoded.length, RESPONSE_BYTES - 1);
    respBuf.fill(0);
    respBuf.set(encoded.subarray(0, writeLen));
    Atomics.store(control, 0, 2);
    Atomics.notify(control, 0, 1);
}

function showModalPrompt(promptText) {
    modalPrompt.textContent = promptText || 'Input:';
    modalInput.value = '';
    modal.style.display = 'flex';
    modalInput.focus();

    const cleanup = () => { 
        modal.style.display = 'none';
        modalOk.removeEventListener('click', onOk);
    };
    const onOk = () => { cleanup(); writeResponseAndWake(modalInput.value || ''); lastRequestId = null; };

    modalOk.addEventListener('click', onOk);
    modalInput.addEventListener('keydown', function onKey(e) { 
        if (e.key === 'Enter') { 
            e.preventDefault();
            onOk();
            modalInput.removeEventListener('keydown', onKey);
        }
    });
}

function appendOutput(text) {
    terminal.innerHTML += text + '\n';
    terminal.scrollTop = terminal.scrollHeight;
}

/* README rendering + anchor scrolling inside docsContainer */
const docsContainer = document.getElementById('docsContainer');

function slugify(text) {
    return text
        .toString()
        .toLowerCase()
        .trim()
        .replace(/\s+/g, '-')       // spaces -> dashes
        .replace(/[^\w\-]+/g, '')   // remove non-word chars
}

function scrollToIdInDocs(id, { behavior = 'smooth', offset = 8 } = {}) {
    if (!id) return false;
    // use CSS.escape for safety
    const selector = '#' + CSS.escape(id);
    const target = docsContainer.querySelector(selector);
    if (!target) return false;

    // compute scrollTop relative to docsContainer
    const containerRect = docsContainer.getBoundingClientRect();
    const targetRect = target.getBoundingClientRect();
    const relativeTop = targetRect.top - containerRect.top + docsContainer.scrollTop;

    docsContainer.scrollTo({
        top: Math.max(0, relativeTop - offset),
        behavior,
    });
    return true;
}

function attachDocsAnchorHandler() {
    // Intercept clicks on links inside docsContainer
    docsContainer.addEventListener('click', (ev) => {
        const a = ev.target.closest('a');
        if (!a) return;
        const href = a.getAttribute('href') || '';

        // #fragment links
        if (href.startsWith('#')) {
            ev.preventDefault();
            const id = href.slice(1);
            if (scrollToIdInDocs(id)) {
                // update url hash without native jump
                try { history.replaceState(null, '', href); } catch (e) {}
            }
            return;
        }

        // links with same-page + hash (e.g. /path/page.html#section or full URL)
        try {
            const url = new URL(href, location.href);
            const isSamePage = (url.pathname === location.pathname && url.search === location.search);
            if (isSamePage && url.hash) {
                ev.preventDefault();
                const id = url.hash.slice(1);
                if (scrollToIdInDocs(id)) {
                    try { history.replaceState(null, '', url.hash); } catch (e) {}
                }
            }
        } catch (err) {
            // ignore invalid URLs
        }
    });
}

async function loadReadme() {
    try {
        const resp = await fetch('./pkg/README.md');
        if (!resp.ok) {
            docsContainer.innerHTML = `<p class="muted">README.md not found: ${resp.status}</p>`;
            return;
        }
        const md = await resp.text();

        const rawHtml = window.marked.parse(md);
        docsContainer.innerHTML = rawHtml.replace(/<script[\s\S]*?>[\s\S]*?<\/script>/gi, '');

        // Ensure headings have stable IDs (use existing id if present)
        docsContainer.querySelectorAll('h1,h2,h3,h4,h5,h6').forEach(h => {
            if (!h.id) {
                const candidate = slugify(h.textContent || h.innerText || 'section');
                // ensure uniqueness
                let id = candidate;
                let i = 1;
                while (docsContainer.querySelector('#' + CSS.escape(id))) {
                    id = `${candidate}-${i++}`;
                }
                h.id = id;
            }
        });

        // Attach click handler once (idempotent)
        if (!docsContainer._anchorsAttached) {
            attachDocsAnchorHandler();
            docsContainer._anchorsAttached = true;
        }

        // If the current page URL already has a hash, scroll to it after rendering
        const currentHash = window.location.hash;
        if (currentHash) {
            // small timeout to allow layout / fonts to settle
            setTimeout(() => {
                scrollToIdInDocs(currentHash.slice(1), { behavior: 'auto', offset: 8 });
            }, 60);
        }
    } catch (err) {
        docsContainer.innerHTML = `<p class="muted">Failed to load docs: ${err.message}</p>`;
    }
}

await loadReadme();