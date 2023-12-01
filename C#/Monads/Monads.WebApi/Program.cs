using OpResult = Result<double, MathError>;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddEndpointsApiExplorer();
builder.Services.AddSwaggerGen();

var app = builder.Build();

if (app.Environment.IsDevelopment())
{
    app.UseSwagger();
    app.UseSwaggerUI();
}

app.UseWhen(ctx => !ctx.Request.Path.StartsWithSegments("/requests"), builder => 
{
    builder.Use((ctx, next) =>
    {
        var numeroRequest = RequestsCounter.Increase();

        if (numeroRequest % 5 == 0)
        {
            throw new Exception("nao pode...");
        }

        return next(ctx);
    });
});

app.MapGet("/requests", () => RequestsCounter.Count);

app.MapPost("/party", (RequestPeople request) =>
{
    return EnterParty(request).Match(s => Results.Ok(s), e => e switch
    {
        TroubleMakerError t => Results.Problem($"SAI DAQUI {t.TroubleMakerName}"),
        GabrielError => Results.Forbid(),
        UnderageError => Results.UnprocessableEntity(e),
        _ => throw new System.Diagnostics.UnreachableException()
    });
});

app.MapPost("/calculation", (CalculationRequest request) =>
{
    static OpResult Sum(OpResult a, OpResult b) => a.And(a => b.Map(b => a + b));
    static OpResult Sub(OpResult a, OpResult b) => a.And(a => b.Map(b => a - b));
    static OpResult Mul(OpResult a, OpResult b) => a.And(a => b.Map(b => a * b));
    static OpResult Div(OpResult a, OpResult b) => a.And(a => b.Map(b => a / b));

    var result = request.Calculations.Aggregate(new OpResult(0), (acc, calc) =>
    {
        return Sum(acc, calc.Calc switch
        {
            "sum" => Sum(calc.A, calc.B),
            "sub" => Sub(calc.A, calc.B),
            "mul" => Mul(calc.A, calc.B),
            "div" => Div(calc.A, calc.B),
            _ => new InvalidOperationError(calc.Calc),
        });
    });

    return result.Match(s => Results.Ok(s), e => TypedResults.BadRequest(e.Message));
});

app.Run();

static Result<ResponsePeople, ValidationError> EnterParty(RequestPeople request)
{
    if (request.Age < 18)
    {
        return new UnderageError();
    }

    if (request.Name.StartsWith("Gabriel"))
    {
        return new GabrielError();
    }

    if (request.TroubleMaker)
    {
        return new TroubleMakerError(request.Name);
    }

    return new ResponsePeople
    {
        Viado = request.TroubleMaker,
        Age = request.Age,
        Name = request.Name,
    };
}

static class RequestsCounter
{
    internal static uint Count = 0;

    internal static uint Increase()
    {
        return Interlocked.Increment(ref Count);
    }
}

class CalculationRequest
{
    public record Calculation(double A, double B, string Calc);
    public required Calculation[] Calculations { get; set; }
}

class MathError(string message)
{
    public readonly string Message = message;
}

class InvalidOperationError(string calc) : MathError($"Não sei resolver conta de {calc}...") { }

class DivisionByZeroError() : MathError("Não da pra dividir por zero...") { }

abstract class Error(string message)
{
    public readonly string Message = message;
}

class ValidationError(string message) : Error(message) { }

class GabrielError() : ValidationError("Não pode Gabriel na festa");

class UnderageError() : ValidationError("Não pode menor de idade na festa");

class TroubleMakerError(string troubleMaker) : ValidationError("Não pode encrenqueiro na festa")
{
    public string TroubleMakerName { get; } = troubleMaker;
}

class ResponsePeople
{
    public required int Age { get; set; }
    public required bool Viado { get; set; }
    public required string Name { get; set; }
}

class RequestPeople
{
    public required string Name { get; set; }
    public required bool TroubleMaker { get; set; }
    public required int Age { get; set; }
}

class Result<TSuccess, TFailure>
{
    public Result(TSuccess success)
    {
        Succeded = true;
        Success = success;
    }

    public Result(TFailure error)
    {
        Succeded = false;
        Error = error;
    }

    public bool Succeded { get; }
    public TSuccess? Success { get; }
    public TFailure? Error { get; }

    public static implicit operator Result<TSuccess, TFailure>(TSuccess success) => new(success);

    public static implicit operator Result<TSuccess, TFailure>(TFailure failure) => new(failure);

    public Result<T, TFailure> Map<T>(Func<TSuccess, T> transform)
    {
        return Succeded ? new(transform(Success!)) : new(Error!);
    }

    public Result<TSuccess, T> MapError<T>(Func<TFailure, T> transform)
    {
        return Succeded ? new(Success!) : new(transform(Error!));
    }

    public Result<T, TFailure> And<T>(Func<TSuccess, Result<T, TFailure>> transform)
    {
        return Succeded ? transform(Success!) : new(Error!);
    }

    public T Match<T>(Func<TSuccess, T> success, Func<TFailure, T> error)
    {
        return Succeded ? success(Success!) : error(Error!);
    }
}