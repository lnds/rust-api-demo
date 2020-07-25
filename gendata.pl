my $limit = 20000000;
print("id,value\n");
my $hash;
for (my $i = 0; $i < $limit; $i++) {
	my $value = int(rand(256));
	print("$i,$value\n");
}
