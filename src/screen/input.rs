pub struct Input {
    pub Mouse_X: i32,
    pub Mouse_Y: i32,
    pub Mouse_Left: bool,
    pub Mouse_Right: bool,
    pub Mouse_Middle: bool,

    pub A: bool,
    pub B: bool,
    pub C: bool,
    pub D: bool,
    pub E: bool,
    pub F: bool,
    pub G: bool,
    pub H: bool,
    pub I: bool,
    pub J: bool,
    pub K: bool,
    pub L: bool,
    pub M: bool,
    pub N: bool,
    pub O: bool,
    pub P: bool,
    pub Q: bool,
    pub R: bool,
    pub S: bool,
    pub T: bool,
    pub U: bool,
    pub V: bool,
    pub W: bool,
    pub X: bool,
    pub Y: bool,
    pub Z: bool,
    pub ArrowUp: bool,
    pub ArrowDown: bool,
    pub ArrowLeft: bool,
    pub ArrowRight: bool,
    pub Num0: bool,
    pub Num1: bool,
    pub Num2: bool,
    pub Num3: bool,
    pub Num4: bool,
    pub Num5: bool,
    pub Num6: bool,
    pub Num7: bool,
    pub Num8: bool,
    pub Num9: bool,
    pub Ctrl: bool,
    pub Alt: bool,
    pub Shift: bool,
    pub CapsLock: bool,
    pub Tab: bool,
    pub F1: bool,
    pub F2: bool,
    pub F3: bool,
    pub F4: bool,
    pub F5: bool,
    pub F6: bool,
    pub F7: bool,
    pub F8: bool,
    pub F9: bool,
    pub F10: bool,
    pub F11: bool,
    pub F12: bool,
    pub Esc: bool,
    pub Space: bool,
    pub Enter: bool,
    pub Minus: bool,
    pub Plus: bool,
    pub Section: bool,
    pub Backtick: bool,
}
impl Input {
    pub fn new() -> Input {
        return Input {
            Mouse_X: 0,
            Mouse_Y: 0,
            Mouse_Left: false,
            Mouse_Right: false,
            Mouse_Middle: false,
            A: false,
            B: false,
            C: false,
            D: false,
            E: false,
            F: false,
            G: false,
            H: false,
            I: false,
            J: false,
            K: false,
            L: false,
            M: false,
            N: false,
            O: false,
            P: false,
            Q: false,
            R: false,
            S: false,
            T: false,
            U: false,
            V: false,
            W: false,
            X: false,
            Y: false,
            Z: false,
            ArrowUp: false,
            ArrowDown: false,
            ArrowLeft: false,
            ArrowRight: false,
            Num0: false,
            Num1: false,
            Num2: false,
            Num3: false,
            Num4: false,
            Num5: false,
            Num6: false,
            Num7: false,
            Num8: false,
            Num9: false,
            Ctrl: false,
            Alt: false,
            Shift: false,
            CapsLock: false,
            Tab: false,
            F1: false,
            F2: false,
            F3: false,
            F4: false,
            F5: false,
            F6: false,
            F7: false,
            F8: false,
            F9: false,
            F10: false,
            F11: false,
            F12: false,
            Esc: false,
            Space: false,
            Enter: false,
            Minus: false,
            Plus: false,
            Section: false,
            Backtick: false,
        }
    }
    pub fn zeros(&mut self) {
        self.Mouse_Left = false;
        self.Mouse_Right = false;
        self.Mouse_Middle = false;
        self.A = false;
        self.B = false;
        self.C = false;
        self.D = false;
        self.E = false;
        self.F = false;
        self.G = false;
        self.H = false;
        self.I = false;
        self.J = false;
        self.K = false;
        self.L = false;
        self.M = false;
        self.N = false;
        self.O = false;
        self.P = false;
        self.Q = false;
        self.R = false;
        self.S = false;
        self.T = false;
        self.U = false;
        self.V = false;
        self.W = false;
        self.X = false;
        self.Y = false;
        self.Z = false;
        self.ArrowUp = false;
        self.ArrowDown = false;
        self.ArrowLeft = false;
        self.ArrowRight = false;
        self.Num0 = false;
        self.Num1 = false;
        self.Num2 = false;
        self.Num3 = false;
        self.Num4 = false;
        self.Num5 = false;
        self.Num6 = false;
        self.Num7 = false;
        self.Num8 = false;
        self.Num9 = false;
        self.Ctrl = false;
        self.Alt = false;
        self.Shift = false;
        self.CapsLock = false;
        self.Tab = false;
        self.F1 = false;
        self.F2 = false;
        self.F3 = false;
        self.F4 = false;
        self.F5 = false;
        self.F6 = false;
        self.F7 = false;
        self.F8 = false;
        self.F9 = false;
        self.F10 = false;
        self.F11 = false;
        self.F12 = false;
        self.Esc = false;
        self.Space = false;
        self.Enter = false;
        self.Minus = false;
        self.Plus = false;
        self.Section = false;
        self.Backtick = false;
    }
}