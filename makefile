dev:
	cd website && pnpm run dev

test:
	cd tml && cargo test
	
update:
	cd tml && cargo update
	cd website && pnpm update -L
	
deploy:
	cd tml && cargo install --path . --bin tml --features build-binary 
	cd website && pnpm run build && vercel --prod