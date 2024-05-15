function Controller() {
    // Errors regarding shortcut creation and driver installation are expected and ignored.
    installer.setMessageBoxAutomaticAnswer("installationErrorWithCancel", QMessageBox.Ignore);
    // Set target directory page visibily to false. The headless install will fail freeze it is visible.
    installer.setDefaultPageVisible(QInstaller.TargetDirectory, false)

    installer.installationFinished.connect(function() {
        gui.clickButton(buttons.NextButton);
    })
}


Controller.prototype.IntroductionPageCallback = function() {
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.TargetDirectoryPageCallback = function()
{
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.ComponentSelectionPageCallback = function() {
    // Default components are fine.
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.LicenseAgreementPageCallback = function() {
    gui.currentPageWidget().AcceptLicenseCheckBox.setChecked(true);
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.StartMenuDirectoryPageCallback = function() {
    gui.clickButton(buttons.NextButton);
}

Controller.prototype.ReadyForInstallationPageCallback = function(){
    // Overwrite TargetDir parameter right before installation. Doing it before the TargetDirectoryPageCallback leads to a wrong target directory.
    installer.setValue("TargetDir","C:/DAS64");
    gui.clickButton(buttons.NextButton);
}

// required
Controller.prototype.FinishedPageCallback = function() {
    gui.clickButton(buttons.FinishButton);
}
