$baseUri = 'https://api.ntppool.org/zone'

function Get-NtpPoolZones {
    param(
        [Parameter(Mandatory)]
        [string] $Zone,

        [Collections.Generic.HashSet[string]] $Visited = (
            [Collections.Generic.HashSet[string]]::new()
        )
    )

    if (-not $Visited.Add($Zone)) {
        return
    }

    # Return this zone too.
    $Zone

    $html = (Invoke-WebRequest "$baseUri/$Zone").Content

    $children = [regex]::Matches(
        $html,
        'href="/zone/([^"]+)"'
    ) |
        ForEach-Object { $_.Groups[1].Value } |
        Where-Object {
            $_ -ne $Zone -and
            $_ -notmatch '^@'
        } |
        Sort-Object -Unique

    foreach ($child in $children) {
        Get-NtpPoolZones -Zone $child -Visited $Visited
    }
}

Get-NtpPoolZones '@' |
    Sort-Object -Unique |
    Where-Object { $_ -ne '@' } |
    ForEach-Object {
        "2.$_.pool.ntp.org"
    }
