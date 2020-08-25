require "fileutils"

desc "dev server"
task :dev do
  system "systemfd --no-pid -s http::3000 -- cargo watch -x run"
end

desc "build"
task :build do
  system "docker-compose -f docker-compose.yml build"
end

desc "up"
task :up do
  system "docker-compose -f docker-compose.yml up -d"
end

desc "查看docker容器"
task :ps do
  system "docker-compose -f docker-compose.yml ps"
end

desc "查看docker日志"
task :log do
  ARGV.each { |a| task a.to_sym do ; end }
  system "echo \'docker-compose -f docker-compose.yml  logs -f #{ARGV.drop(1).join(' ')}\'"
  system "docker-compose -f docker-compose.yml logs -f #{ARGV.drop(1).join(' ')}"
end

desc "重启服务"
task :restart do
  ARGV.each { |a| task a.to_sym do ; end }
  system "echo \'docker-compose -f docker-compose.yml restart #{ARGV.drop(1).join(' ')}\'"
  system "docker-compose -f docker-compose.yml restart #{ARGV.drop(1).join(' ')}"
end

desc "暂停docker"
task :stop do
  ARGV.each { |a| task a.to_sym do ; end }
  system "echo \'docker-compose -f docker-compose.yml stop #{ARGV.drop(1).join(' ')}\'"
  system "docker-compose -f docker-compose.yml stop #{ARGV.drop(1).join(' ')}"
end

desc "终止docker"
task :down do
  system "docker-compose -f docker-compose.yml down"
end

desc "登录容器"
task :ssh do
  ARGV.each { |a| task a.to_sym do ; end }
  container = ARGV[1]
  shell = ARGV[2] == nil ? 'bash' : ARGV[2] 

  system "docker-compose -f docker-compose.yml exec #{container} #{shell}"
end
