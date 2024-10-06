use pyo3::prelude::*;
use pyo3::types::IntoPyDict;

#[pyfunction]
pub fn message() -> PyResult<()> {
    // Initialize the Python interpreter
    Python::with_gil(|py| {
        let code = r#"

"""User interface functionality.
This refers to the user interface presented by the screen reader alone, not the graphical user interface.
See L{gui} for the graphical user interface.
"""

import os
import sys
from ctypes import (
	windll,
	oledll,
	byref,
	POINTER
)
import comtypes.client
from comtypes import IUnknown
from comtypes import automation 
from comtypes import COMError
from html import escape
from logHandler import log
import gui
import speech
import braille
from config.configFlags import TetherTo
import globalVars
from typing import Optional

from utils.security import isRunningOnSecureDesktop


# From urlmon.h
URL_MK_UNIFORM = 1

# Dialog box properties
DIALOG_OPTIONS = "resizable:yes;help:no"

#dwDialogFlags for ShowHTMLDialogEx from mshtmhst.h
HTMLDLG_NOUI = 0x0010 
HTMLDLG_MODAL = 0x0020 
HTMLDLG_MODELESS = 0x0040 
HTMLDLG_PRINT_TEMPLATE = 0x0080 
HTMLDLG_VERIFY = 0x0100 


def _warnBrowsableMessageNotAvailableOnSecureScreens(title: Optional[str]) -> None:
	"""Warn the user that a browsable message could not be shown on a secure screen (sign-on screen / UAC
	prompt).
	@param title: If provided, the title of the browsable message to give the user more context.
	"""
	log.warning(
		"While on secure screens browsable messages can not be used."
		" The browsable message window creates a security risk."
		f" Attempted to open message with title: {title!r}"
	)

	if not title:
		browsableMessageUnavailableMsg: str = _(
			# Translators: This is the message for a warning shown if NVDA cannot open a browsable message window
			# when Windows is on a secure screen (sign-on screen / UAC prompt).
			"This feature is unavailable while on secure screens such as the sign-on screen or UAC prompt."
		)
	else:
		browsableMessageUnavailableMsg: str = _(
			# Translators: This is the message for a warning shown if NVDA cannot open a browsable message window
			# when Windows is on a secure screen (sign-on screen / UAC prompt). This prompt includes the title
			# of the Window that could not be opened for context.
			# The {title} will be replaced with the title.
			# The title may be something like "Formatting".
			"This feature ({title}) is unavailable while on secure screens"
			" such as the sign-on screen or UAC prompt."
		)
		browsableMessageUnavailableMsg = browsableMessageUnavailableMsg.format(title=title)

	import wx  # Late import to prevent circular dependency.
	import gui  # Late import to prevent circular dependency.
	log.debug("Presenting browsable message unavailable warning.")
	gui.messageBox(
		browsableMessageUnavailableMsg,
		# Translators: This is the title for a warning dialog, shown if NVDA cannot open a browsable message
		# dialog.
		caption=_("Feature unavailable."),
		style=wx.ICON_ERROR | wx.OK,
	)


def browseableMessage(message: str, title: Optional[str] = None, isHtml: bool = False) -> None:
	"""Present a message to the user that can be read in browse mode.
	The message will be presented in an HTML document.
	@param message: The message in either html or text.
	@param title: The title for the message.
	@param isHtml: Whether the message is html
	"""
	if isRunningOnSecureDesktop():
		import wx  # Late import to prevent circular dependency.
		wx.CallAfter(_warnBrowsableMessageNotAvailableOnSecureScreens, title)
		return

	htmlFileName = os.path.join(globalVars.appDir, 'message.html')
	if not os.path.isfile(htmlFileName ): 
		raise LookupError(htmlFileName )
	moniker = POINTER(IUnknown)()
	windll.urlmon.CreateURLMonikerEx(0, htmlFileName, byref(moniker), URL_MK_UNIFORM)
	if not title:
		# Translators: The title for the dialog used to present general NVDA messages in browse mode.
		title = _("NVDA Message")
	if not isHtml:
		message = f"<pre>{escape(message)}</pre>"
	try:
		d = comtypes.client.CreateObject("Scripting.Dictionary")
	except (COMError, OSError):
		log.error("Scripting.Dictionary component unavailable", exc_info=True)
		# Store the module level message function in a new variable since it is masked by a local variable with
		# the same name
		messageFunction = globals()['message']
		# Translators: reported when unable to display a browsable message.
		messageFunction(_("Unable to display browseable message"))
		return
	d.add("title", title)
	d.add("message", message)
	dialogArgsVar = automation.VARIANT(d)
	gui.mainFrame.prePopup() 
	windll.mshtml.ShowHTMLDialogEx( 
		gui.mainFrame.Handle , 
		moniker , 
		HTMLDLG_MODELESS , 
		byref(dialogArgsVar), 
		DIALOG_OPTIONS, 
		None
	)
	gui.mainFrame.postPopup() 


def message(
		text: str,
		speechPriority: Optional[speech.Spri] = None,
		brailleText: Optional[str] = None,
):
	"""Present a message to the user.
	The message will be presented in both speech and braille.
	@param text: The text of the message.
	@param speechPriority: The speech priority.
	@param brailleText: If specified, present this alternative text on the braille display.
	"""
	speech.speakMessage(text, priority=speechPriority)
	braille.handler.message(brailleText if brailleText is not None else text)


def reviewMessage(text: str, speechPriority: Optional[speech.Spri] = None):
	"""Present a message from review or object navigation to the user.
	The message will always be presented in speech, and also in braille if it is tethered to review or when auto tethering is on.
	@param text: The text of the message.
	@param speechPriority: The speech priority.
	"""
	speech.speakMessage(text, priority=speechPriority)
	if braille.handler.shouldAutoTether or braille.handler.getTether() == TetherTo.REVIEW:
		braille.handler.message(text)


def reportTextCopiedToClipboard(text: Optional[str] = None):
	"""Notify about the result of a "Copy to clipboard" operation.
	@param text: The text that has been copied. Set to `None` to notify of a failed operation.
	See: `api.copyToClip`
	"""
	if not text:
		# Translators: Presented when unable to copy to the clipboard because of an error
		# or the clipboard content did not match what was just copied.
		message(_("Unable to copy"))
		return
	# Depending on the speech synthesizer, large amount of spoken text can freeze NVDA (#11843)
	textLength = len(text)
	if textLength < 1024:
		spokenText = text
	else:
		# Translators: Spoken instead of a lengthy text when copied to clipboard.
		spokenText = ngettext("%d character", "%d characters", textLength) % textLength
	message(
		# Translators: Announced when a text has been copied to clipboard.
		# {text} is replaced by the copied text.
		text=_("Copied to clipboard: {text}").format(text=spokenText),
		# Translators: Displayed in braille when a text has been copied to clipboard.
		# {text} is replaced by the copied text.
		brailleText=_("Copied: {text}").format(text=text)
	)

                "#;
        // Import the Python module (example.py should be in the working directory)
        let ui = PyModule::import_bound(py, "ui")?;
        // let ui = PyModule::from_code_bound(py, code, "rs_kernel.py", "kernel_ui")?;
        let _message: PyObject = ui.getattr("message")?.call1(("hello world from rust", ))?.into();

        // Get the class `MyClass` from the module
        // let my_class = example.getattr("MyClass")?;


        // Create an instance of `MyClass` with parameters 5 and 10
        // let instance = my_class.call1((5, 10))?;

        // Call the `add` method of the instance
        // let result: i32 = instance.call_method0("add")?.extract()?;

        // println!("The result of add is: {}", result); // Output: The result of add is: 15

        Ok(())
    })
}
