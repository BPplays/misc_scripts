$api = 'https://api.ntppool.org/zone'

function Get-NtpZones {
    param(
        [string] $Zone = '@'
    )

    $html = (Invoke-RestMethod "$api/$Zone").ToString()
	#Write-Output $html

    # The API response contains links like:
    #   /zone/north-america
    #   /zone/us
    [regex]::Matches($html, 'href="/zone/([^"]+)"') |
        ForEach-Object { $_.Groups[1].Value } |
        Sort-Object -Unique
}

$zones = Get-NtpZones |
    ForEach-Object {
        $zone = $_

        # Get each region's children
        $children = Get-NtpZones $zone

        # Keep the region itself
        $zone

        # And its countries
        $children
    } |
    Where-Object { $_ -ne '@' } |
    Sort-Object -Unique

$zones | ForEach-Object {
    "2.$_.pool.ntp.org"
}
