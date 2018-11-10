my StartLog()
tell application "Swinsian"
	(*activate*)
	set the script_text to ""
	tell application "Script Editor"
		"hello there!"
	end tell
	
	set selected to selection of window 1
	repeat with t in selected
		my WriteLog("Track Name: " & name of t)
		my WriteLog("	Path: " & path of t)
		set pp to path of t
		set qpath to quoted form of pp
		set scr to "export PATH=$PATH:/usr/local/bin/:/Users/adam/projects/bellson/bin/; ~/projects/ellington/target/release/ellington oneshot --audiofile " & qpath
		if comment of t is missing value then
			my WriteLog("	Comment: no comment in track")
		else
			my WriteLog("	Comment: " & comment of t)
			set com to comment of t
			set qcomment to quoted form of com
			set scr to scr & " --comment " & qcomment
		end if
		my WriteLog("	scr: " & scr)
		set script_result to do shell script scr
		my WriteLog("	scr_res: " & script_result)
		set the comment of t to script_result
	end repeat
end tell

on StartLog()
	set this_file to (((path to desktop folder) as text) & "swinsianlog.txt")
	my write_to_file("Start of swing log", this_file, false)
end StartLog

on WriteLog(the_text)
	set this_story to the_text & "
"
	set this_file to (((path to desktop folder) as text) & "swinsianlog.txt")
	my write_to_file(this_story, this_file, true)
end WriteLog

on write_to_file(this_data, target_file, append_data) -- (string, file path as string, boolean)
	try
		set the target_file to the target_file as text
		set the open_target_file to Â
			open for access file target_file with write permission
		if append_data is false then Â
			set eof of the open_target_file to 0
		write this_data to the open_target_file starting at eof
		close access the open_target_file
		log this_data
		return true
	on error
		try
			close access file target_file
		end try
		return false
	end try
end write_to_file

