require 'fileutils'

def includes(lesson) = case lesson
	when "lesson10" then :lesson10
	when "lesson11" then :lesson11
	when "lesson12" then :lesson12
	when "lesson13" then :lesson13
	when "lesson14" then :lesson14
	when "lesson15" then :lesson15
	when "lesson16", "lesson22", "lesson23", "lesson24", "lesson25", "lesson32"
		:lesson16

	when "lesson17" then :lesson17
	when "lesson18" then :lesson18
	when "lesson21" then :lesson21
	when "lesson26", "lesson27", "lesson28", "lesson29", "lesson44"
		:lesson26

	when "lesson30" then :lesson30
	when "lesson33" then :lesson33
	when "lesson35" then :lesson35
	when "lesson38" then :lesson38
	when "lesson39" then :lesson39
	when "lesson40" then :lesson40
	when "lesson41" then :lesson41
	when "lesson42" then :lesson42
	when "lesson45" then :lesson45
	when "lesson46" then :lesson46

	when "lesson47", "lesson48" then :lesson47
	when "lesson49" then :lesson49
end

# active it first
#system('source "$HOME/emsdk/emsdk_env.sh"') # Where your emsdk_env is.
#system('emsdk activate latest')

Dir.children("src").each do |child|
	if child.start_with?("lesson")
		lesson = child.sub(".rs", "")
		extra = includes(lesson).nil? ? "'" : " --preload-file resources/#{includes(lesson)}'"
		
		features = case lesson.match(/\d+/)[0]
		when "38", "47", "48", "49" then '--features="rand"'
		when "50" then '--features="glow glu-sys"' # note: error: undefined symbol: glEnd (referenced by top-level compiled C/C++ code)
		when "51" then '--features="glow"'
		else ""
		end

		out_dir = "public/wasm/#{lesson}"
		FileUtils.mkdir(out_dir)
		cargo = "cargo build --bin #{lesson} --release --target wasm32-unknown-emscripten #{features}"
		system("EMCC_CFLAGS='-o #{out_dir}/#{lesson}.js -s WASM=1 -s USE_SDL=2 -s USE_SDL_IMAGE=2 -s SDL2_IMAGE_FORMATS=\"[\"png\",\"bmp\"]\" -s USE_SDL_TTF=2#{extra} #{cargo}")

	end
end