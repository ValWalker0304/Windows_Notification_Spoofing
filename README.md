# A study on UI redressing(spoofing) in Windows 11
A dive into how the Windows Runtime (WinRT) notification infrastructure can be utilized for social engineering attacks by impersonating trusted System AppIDs.


## The Problem
Windows allows standard users to invoke notifications to the system tray using any `AppId` and the `ToastNotifcationManager` without signature verification of the sender.

## Possible attack paths:
* Vector 1:
  * Attacker executes a low level binary
  * This binary sends a toast impersonating notification as a trusted component
  * User clicks a button with a protocol activation, launching a malicious URL or pre-installed backdoor handler
* Vector 2:
  * Attacker could fabricate urgency by prompting the user that they will lose access to their system account
  * The user clicks on the button to follow but is led to a third party "copy-cat" website for information extraction.

## Mitigation Suggestions: 
* All system notifications should be matched between sender and display.
* System AppId's should be randomized thus making spoofing a notification from an app nearly impossible.