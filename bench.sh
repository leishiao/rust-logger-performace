	COUNT=10000000
    time ./target/release/perf-log -i flexi_file -c ${COUNT}
	time ./target/release/perf-log -i flexi_roll -c ${COUNT}
	time ./target/release/perf-log -i log4rs_file -c ${COUNT}
	time ./target/release/perf-log -i log4rs_roll -c ${COUNT}