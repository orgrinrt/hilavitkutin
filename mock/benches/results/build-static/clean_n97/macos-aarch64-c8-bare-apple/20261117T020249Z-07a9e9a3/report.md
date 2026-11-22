# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (101996.65 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 101996653583.3 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 101996653583.3 ns, slowest 106876505520.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: every crate built into an empty target directory |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 106970814056ns | 106876514979ns | 100058314542ns | 105885768903ns | 112054631542ns | +4.79% |
| llvm_non_incremental | 102080863639ns | 101996660104ns | 99007480042ns | 101056270875ns | 105154444583ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 106970804806ns | 100058300583ns | 112054625750ns | +4.79% |
| llvm_non_incremental | 102080857660ns | 99007474833ns | 105154438438ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 106970814056ns | 106970814056ns | +4.79% |
| llvm_non_incremental | 102080863639ns | 102080863639ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 101996653583ns | base | --- | [99091480958, 105154438438] | --- | --- | --- | --- |
| llvm_incremental | 106876505521ns | +4717759333.3ns (+4.6%) | [+345010958, +9607071146]ns | [101981283146, 112054625750] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 99420530708ns | +0.6% |
| 1 | 2 | 99007474833ns | +4.9% |
| 1 | 3 | 104589254750ns | +0.0% |
| 1 | 4 | 99175487083ns | +13.8% |
| 1 | 5 | 104572776458ns | +4.3% |
| 1 | 6 | 105719622125ns | +5.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | 0.302 | moderate+ |
| llvm_non_incremental | -0.098 | ok |

**Consistency summary:**

- **llvm_incremental**: won 0/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 2736887576.4ns | 106970804805.6ns | 2.6% |  |
| llvm_non_incremental | 467558187.5ns | 102080857659.7ns | 0.5% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 100058300583.3-112054625750.0 ns)
  100058300583.3 |########################################
  100658116841.6 |
  101257933100.0 |
  101857749358.3 |
  102457565616.6 |
  103057381875.0 |
  103657198133.3 |########################################
  104257014391.6 |########################################
  104856830650.0 |
  105456646908.3 |
  106056463166.6 |
  106656279425.0 |
  107256095683.3 |
  107855911941.7 |
  108455728200.0 |
  109055544458.3 |########################################
  109655360716.7 |
  110255176975.0 |
  110854993233.3 |########################################
  111454809491.7 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 99007474833.3-105154438437.5 ns)
  99007474833.3 |########################################
  99314823013.5 |####################
  99622171193.7 |
  99929519373.9 |
  100236867554.1 |
  100544215734.4 |
  100851563914.6 |
  101158912094.8 |
  101466260275.0 |
  101773608455.2 |
  102080956635.4 |
  102388304815.6 |
  102695652995.8 |
  103003001176.0 |
  103310349356.2 |
  103617697536.4 |
  103925045716.7 |
  104232393896.9 |
  104539742077.1 |########################################
  104847090257.3 |
  (0 below, 1 above range)

```
