use std::*;
use std::collections::HashMap;

struct pypilotScopeBase {
    m_splitter1: ST0,
    m_panel2: ST1,
    clValues: ST2,
    glpanel: ST3,
    glArea: ST4,
    bZero: ST5,
    bCenter: ST6,
    bScalePlus: ST7,
    bScaleMinus: ST8,
    bOffsetPlus: ST9,
    bOffsetMinus: ST10,
    tbFreeze: ST11,
    bReset: ST12,
    cbfftw: ST13,
    m_staticText1: ST14,
    sTime: ST15,
    bClose: ST16,
}

impl pypilotScopeBase {
    fn __init__<T0>(&self, parent: T0) {
        wx.Frame.__init__(self, parent, wx.ID_ANY, _("pypilot Scope"), wx.DefaultPosition, wx.Size(1024, 600), (wx.DEFAULT_FRAME_STYLE | wx.TAB_TRAVERSAL));
        self.SetSizeHints(wx.DefaultSize, wx.DefaultSize);
        let fgSizer3 = wx.FlexGridSizer(0, 1, 0, 0);
        fgSizer3.AddGrowableCol(0);
        fgSizer3.AddGrowableRow(0);
        fgSizer3.SetFlexibleDirection(wx.BOTH);
        fgSizer3.SetNonFlexibleGrowMode(wx.FLEX_GROWMODE_SPECIFIED);
        self.m_splitter1 = wx.SplitterWindow(self, wx.ID_ANY, wx.DefaultPosition, wx.DefaultSize, wx.SP_3D);
        self.m_splitter1.Bind(wx.EVT_IDLE, self.m_splitter1OnIdle);
        self.m_panel2 = wx.Panel(self.m_splitter1, wx.ID_ANY, wx.DefaultPosition, wx.DefaultSize, wx.TAB_TRAVERSAL);
        let fgSizer51 = wx.FlexGridSizer(0, 2, 0, 0);
        fgSizer51.AddGrowableCol(0);
        fgSizer51.AddGrowableRow(0);
        fgSizer51.SetFlexibleDirection(wx.BOTH);
        fgSizer51.SetNonFlexibleGrowMode(wx.FLEX_GROWMODE_SPECIFIED);
        let clValuesChoices = vec![];
        self.clValues = wx.CheckListBox(self.m_panel2, wx.ID_ANY, wx.DefaultPosition, wx.DefaultSize, clValuesChoices, 0);
        fgSizer51.Add(self.clValues, 0, (wx.ALL | wx.EXPAND), 5);
        self.m_panel2.SetSizer(fgSizer51);
        self.m_panel2.Layout();
        fgSizer51.Fit(self.m_panel2);
        self.glpanel = wx.Panel(self.m_splitter1, wx.ID_ANY, wx.DefaultPosition, wx.DefaultSize, wx.TAB_TRAVERSAL);
        let fgSizer41 = wx.FlexGridSizer(0, 2, 0, 0);
        fgSizer41.AddGrowableCol(0);
        fgSizer41.AddGrowableRow(0);
        fgSizer41.SetFlexibleDirection(wx.BOTH);
        fgSizer41.SetNonFlexibleGrowMode(wx.FLEX_GROWMODE_SPECIFIED);
        self.glArea = wx.glcanvas.GLCanvas(self.glpanel);
        fgSizer41.Add(self.glArea, 0, (wx.ALL | wx.EXPAND), 5);
        self.glpanel.SetSizer(fgSizer41);
        self.glpanel.Layout();
        fgSizer41.Fit(self.glpanel);
        self.m_splitter1.SplitVertically(self.m_panel2, self.glpanel, 250);
        fgSizer3.Add(self.m_splitter1, 1, wx.EXPAND, 5);
        let fgSizer5 = wx.FlexGridSizer(1, 0, 0, 0);
        fgSizer5.SetFlexibleDirection(wx.BOTH);
        fgSizer5.SetNonFlexibleGrowMode(wx.FLEX_GROWMODE_SPECIFIED);
        self.bZero = wx.Button(self, wx.ID_ANY, _("Zero"), wx.DefaultPosition, wx.DefaultSize, 0);
        fgSizer5.Add(self.bZero, 0, wx.ALL, 5);
        self.bCenter = wx.Button(self, wx.ID_ANY, _("Center"), wx.DefaultPosition, wx.DefaultSize, 0);
        fgSizer5.Add(self.bCenter, 0, wx.ALL, 5);
        self.bScalePlus = wx.Button(self, wx.ID_ANY, _("Scale +"), wx.DefaultPosition, wx.DefaultSize, 0);
        fgSizer5.Add(self.bScalePlus, 0, wx.ALL, 5);
        self.bScaleMinus = wx.Button(self, wx.ID_ANY, _("Scale -"), wx.DefaultPosition, wx.DefaultSize, 0);
        fgSizer5.Add(self.bScaleMinus, 0, wx.ALL, 5);
        self.bOffsetPlus = wx.Button(self, wx.ID_ANY, _("Offset /\"), wx.DefaultPosition, wx.DefaultSize, 0);
fgSizer5.Add(self.bOffsetPlus, 0, wx.ALL, 5);
self.bOffsetMinus = wx.Button(self, wx.ID_ANY, _("Offset \ / "), wx.DefaultPosition, wx.DefaultSize, 0);
fgSizer5.Add(self.bOffsetMinus, 0, wx.ALL, 5);
self.tbFreeze = wx.ToggleButton(self, wx.ID_ANY, _("Freeze"), wx.DefaultPosition, wx.DefaultSize, 0);
fgSizer5.Add(self.tbFreeze, 0, wx.ALL, 5);
self.bReset = wx.Button(self, wx.ID_ANY, _("Reset"), wx.DefaultPosition, wx.DefaultSize, 0);
fgSizer5.Add(self.bReset, 0, wx.ALL, 5);
self.cbfftw = wx.CheckBox(self, wx.ID_ANY, _("fftw"), wx.DefaultPosition, wx.DefaultSize, 0);
fgSizer5.Add(self.cbfftw, 0, wx.ALL, 5);
self.m_staticText1 = wx.StaticText(self, wx.ID_ANY, _("Time"), wx.DefaultPosition, wx.DefaultSize, 0);
self.m_staticText1.Wrap(-1);
fgSizer5.Add(self.m_staticText1, 0, (wx.ALIGN_CENTER_VERTICAL | wx.ALL), 5);
self.sTime = wx.SpinCtrl(self, wx.ID_ANY, wx.EmptyString, wx.DefaultPosition, wx.Size(60, -1), wx.SP_ARROW_KEYS, 1, 3600, 0);
fgSizer5.Add(self.sTime, 0, wx.ALL, 5);
self.bClose = wx.Button(self, wx.ID_ANY, _("Close"), wx.DefaultPosition, wx.DefaultSize, 0);
fgSizer5.Add(self.bClose, 0, wx.ALL, 5);
fgSizer3.Add(fgSizer5, 1, wx.EXPAND, 5);
self.SetSizer(fgSizer3);
self.Layout();
self.Centre(wx.BOTH);
self.clValues.Bind(wx.EVT_LISTBOX, self.onValueSelected);
self.clValues.Bind(wx.EVT_CHECKLISTBOX, self.onValueToggled);
self.glArea.Bind(wx.EVT_KEY_DOWN, self.onKeyPress);
self.glArea.Bind(wx.EVT_LEFT_DOWN, self.onMouseEvents);
self.glArea.Bind(wx.EVT_LEFT_UP, self.onMouseEvents);
self.glArea.Bind(wx.EVT_MIDDLE_DOWN, self.onMouseEvents);
self.glArea.Bind(wx.EVT_MIDDLE_UP, self.onMouseEvents);
self.glArea.Bind(wx.EVT_RIGHT_DOWN, self.onMouseEvents);
self.glArea.Bind(wx.EVT_RIGHT_UP, self.onMouseEvents);
self.glArea.Bind(wx.EVT_MOTION, self.onMouseEvents);
self.glArea.Bind(wx.EVT_LEFT_DCLICK, self.onMouseEvents);
self.glArea.Bind(wx.EVT_MIDDLE_DCLICK, self.onMouseEvents);
self.glArea.Bind(wx.EVT_RIGHT_DCLICK, self.onMouseEvents);
self.glArea.Bind(wx.EVT_LEAVE_WINDOW, self.onMouseEvents);
self.glArea.Bind(wx.EVT_ENTER_WINDOW, self.onMouseEvents);
self.glArea.Bind(wx.EVT_MOUSEWHEEL, self.onMouseEvents);
self.glArea.Bind(wx.EVT_PAINT, self.onPaintGL);
self.glArea.Bind(wx.EVT_SIZE, self.onSizeGL);
self.bZero.Bind(wx.EVT_BUTTON, self.onZero);
self.bCenter.Bind(wx.EVT_BUTTON, self.onCenter);
self.bScalePlus.Bind(wx.EVT_BUTTON, self.onScalePlus);
self.bScaleMinus.Bind(wx.EVT_BUTTON, self.onScaleMinus);
self.bOffsetPlus.Bind(wx.EVT_BUTTON, self.onOffsetPlus);
self.bOffsetMinus.Bind(wx.EVT_BUTTON, self.onOffsetMinus);
self.tbFreeze.Bind(wx.EVT_TOGGLEBUTTON, self.onFreeze);
self.bReset.Bind(wx.EVT_BUTTON, self.onReset);
self.sTime.Bind(wx.EVT_SPINCTRL, self.onTime);
self.bClose.Bind(wx.EVT_BUTTON, self.onClose);
}
fn __del__(&self)  {
/*pass*/
}
fn onValueSelected<T0>(&self, event: T0)  {
/*pass*/
}
fn onValueToggled<T0>(&self, event: T0)  {
/*pass*/
}
fn onKeyPress<T0>(&self, event: T0)  {
/*pass*/
}
fn onMouseEvents<T0>(&self, event: T0)  {
/*pass*/
}
fn onPaintGL<T0>(&self, event: T0)  {
/*pass*/
}
fn onSizeGL<T0>(&self, event: T0)  {
/*pass*/
}
fn onZero<T0>(&self, event: T0)  {
/*pass*/
}
fn onCenter<T0>(&self, event: T0)  {
/*pass*/
}
fn onScalePlus<T0>(&self, event: T0)  {
/*pass*/
}
fn onScaleMinus<T0>(&self, event: T0)  {
/*pass*/
}
fn onOffsetPlus<T0>(&self, event: T0)  {
/*pass*/
}
fn onOffsetMinus<T0>(&self, event: T0)  {
/*pass*/
}
fn onFreeze<T0>(&self, event: T0)  {
/*pass*/
}
fn onReset<T0>(&self, event: T0)  {
/*pass*/
}
fn onTime<T0>(&self, event: T0)  {
/*pass*/
}
fn onClose<T0>(&self, event: T0)  {
/*pass*/
}
fn m_splitter1OnIdle<T0>(&self, event: T0)  {
self.m_splitter1.SetSashPosition(250);
self.m_splitter1.Unbind(wx.EVT_IDLE);
} 
}