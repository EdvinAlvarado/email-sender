# If cannot run script, verify the following.
# Set-ExecutionPolicy -ExecutionPolicy Unrestricted
#Import-Module Microsoft.PowerShell.Utility


$emails = $args[0] | ConvertFrom-Json
# Write-Host ($emails | Format-Table | Out-String)
$outlook = New-Object -ComObject Outlook.Application

foreach ($e in $emails) {
    $email = $outlook.CreateItem(0)
    $email.To = $e.to
    $email.CC = $e.cc
    $email.Subject = $e.subject
    $email.Body = $e.body
	if ($e.attachment) {
		$email.Attachments.add($e.attachment)
	}

    $email.Send()
}

#$outlook.Quit()
